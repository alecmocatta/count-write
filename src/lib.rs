// count-write
// Copyright (C) SOFe
//
// Licensed under the Apache License, Version 2.0 (the License);
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an AS IS BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::Write;

type Result<T = (), E = std::io::Error> = std::result::Result<T, E>;

/// A wrapper for [`std::io::Write`][Write] (and other optional Write implementations) that counts
/// the total number of bytes written successfully.
///
/// ```
/// use std::io::{BufWriter, Write};
///
/// use count_write::CountWrite;
///
/// let vec = vec![]; // note that Vec<u8> : Write
/// let cw = CountWrite::from(vec);
/// let mut bw = BufWriter::new(cw);
/// write!(bw, "abc")?;
/// assert_eq!(bw.into_inner()?.count(), 3);
/// # Ok::<(), std::io::Error>(())
/// ```
///
/// If the inner writer does not successfully write all bytes,
/// only the successfully written bytes are counted.
/// (This does not affect users who always use [`write_all`][write_all], [`write!`][write-macro],
/// etc.)
///
/// ```
/// use std::io::{Result, Write};
///
/// use count_write::CountWrite;;
///
/// /// A dummy struct that only accepts half of the input into a Vec.
/// struct OnlyHalf;
///
/// impl Write for OnlyHalf {
///     fn write(&mut self, buf: &[u8]) -> Result<usize> {
///     dbg!(buf);
///         Ok((buf.len() + 1) / 2)
///     }
///
///     fn flush(&mut self) -> Result<()> { Ok(()) }
/// }
///
/// let mut cw = CountWrite::from(OnlyHalf);
/// write!(cw, "abc")?; // Here, we keep retrying writing into the writer
/// assert_eq!(cw.count(), 3);
///
/// let mut cw = CountWrite::from(OnlyHalf);
/// cw.write(b"abc")?; // Here, we only write to the writer once and do not retry
/// assert_eq!(cw.count(), 2);
/// # Ok::<(), std::io::Error>(())
/// ```
///
/// [Write]: https://doc.rust-lang.org/std/io/trait.Write.html
/// [write_all]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
/// [write-macro]: https://doc.rust-lang.org/std/macro.write.html
pub struct CountWrite<W> {
    inner: W,
    count: u64,
}

impl<W> CountWrite<W> {
    /// Returns the number of bytes successfull written so far
    pub fn count(&self) -> u64 {
        self.count
    }

    /// Extracts the inner writer, discarding this wrapper
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W> From<W> for CountWrite<W> {
    fn from(inner: W) -> Self {
        Self { inner, count: 0 }
    }
}

impl<W: Write> Write for CountWrite<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let written = self.inner.write(buf)?;
        self.count += written as u64;
        Ok(written)
    }

    fn flush(&mut self) -> Result {
        self.inner.flush()
    }
}

#[cfg(feature = "futures")]
mod futures;

#[cfg(feature = "tokio")]
mod tokio;
