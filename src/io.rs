#[allow(dead_code)]

pub mod io {
    use std::borrow::Borrow;
    use std::collections::btree_set::Range;
    use std::fmt::Debug;
    use std::str::FromStr;

    pub struct IStream {
        buf: String,
        std_in: bool,
        delim: char,
        ignore_endl: bool,
    }

    impl IStream {
        pub fn new() -> IStream {
            IStream {
                buf: String::new(),
                std_in: true,
                delim: ' ',
                ignore_endl: false,
            }
        }

        /**
         Private function to get input from the standard
         input
        */
        fn consume_stdin(&mut self) -> Option<String> {
            use std::io::{stdin, stdout, Write};
            let _ = stdout().flush();
            match stdin().read_line(&mut self.buf) {
                Ok(_) => Some(self.trim_to_owned(&self.buf)),
                Err(why) => panic!("{}", why),
            }
        }

        /**
         Find the next delimiter or newline in the buffer
        */
        fn next_split(&self) -> usize {
            let ind_of_delim: usize = self.buf.find(self.delim).unwrap_or(self.buf.len());

            if !self.ignore_endl {
                ind_of_delim.min(self.next_endl())
            } else {
                ind_of_delim
            }
        }

        /**
         Take a `&str`, trim it according to the delimiter,
         and return it as an owned value
        */
        fn trim_to_owned(&self, s: &str) -> String {
            if self.delim == ' ' {
                s.trim().to_owned()
            } else {
                s.replace(self.delim, "")
            }
        }

        /**
         Find the next occurrence of a newline in the buffer
        */
        fn next_endl(&self) -> usize {
            self.buf.find('\n').unwrap_or(self.buf.len())
        }

        pub fn ignore_endl(&mut self, ignore: bool) {
            self.ignore_endl = ignore;
        }

        /**
         Function to read the next line from the stream
        */
        pub fn next_line(&mut self) -> Option<String> {
            if self.buf.is_empty() && self.std_in {
                self.consume_stdin()
            } else if !self.buf.is_empty() {
                let nb: String = self.buf.split_off(self.next_endl());
                let line: String = self.buf.trim().to_owned();
                self.buf = self.trim_to_owned(&nb);

                Some(line)
            } else {
                None
            }
        }

        /**
         Function to read the next key in the stream and
         try to convert it to the appropriate value
        */
        pub fn next<T>(&mut self) -> Option<T>
        where
            T: FromStr,
            T: Clone,
            <T as FromStr>::Err: Debug,
        {
            if self.buf.is_empty() && self.std_in {
                self.consume_stdin();
                self.next::<T>()
            } else if !self.buf.is_empty() {
                let nb: String = self.buf.split_off(self.next_split());
                let val: String = self.buf.trim().to_owned();
                self.buf = self.trim_to_owned(&nb);

                match val.parse::<T>() {
                    Ok(t) => Some(t),
                    Err(_) => None,
                }
            } else {
                None
            }
        }

        /**
         Function to read the stream and get the next
         valid `T` key, unless none is present
        */
        pub fn next_valid<T>(&mut self) -> Option<T>
        where
            T: FromStr,
            T: Clone,
            <T as FromStr>::Err: Debug,
        {
            if self.buf.is_empty() && self.std_in {
                self.consume_stdin();
                self.next_valid::<T>()
            } else if !self.buf.is_empty() {
                loop {
                    let nb: String = self.buf.split_off(self.next_split());
                    let val: String = self.buf.trim().to_owned();
                    self.buf = self.trim_to_owned(&nb);

                    let r: Result<_, _> = val.parse::<T>();

                    if r.is_ok() {
                        return Some(r.unwrap());
                    } else if self.buf.trim().is_empty() {
                        return None;
                    }
                }
            } else {
                None
            }
        }

        /**
         Clear the buffer, returning the remaining
         content
        */
        pub fn flush(&mut self) -> String {
            let r = self.buf.clone();
            self.buf.clear();
            r
        }

        /**
         Skip `n` tokens in the buffer
        */
        pub fn skip(&mut self, n: usize) {
            for _ in 0..n {
                let (_, buf) = self.buf.split_at(self.next_split());
                self.buf = self.trim_to_owned(&buf);
            }
        }

        pub fn use_delim(&mut self, delim: char) {
            self.delim = delim;
        }
    }

    impl From<String> for IStream {
        fn from(s: String) -> Self {
            IStream {
                buf: s,
                std_in: false,
                delim: ' ',
                ignore_endl: false,
            }
        }
    }

    impl From<&str> for IStream {
        fn from(s: &str) -> Self {
            IStream {
                buf: String::from(s),
                std_in: false,
                delim: ' ',
                ignore_endl: false,
            }
        }
    }
}
