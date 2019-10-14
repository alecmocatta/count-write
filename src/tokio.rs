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

use core::pin::Pin;
use core::task::{Context, Poll};

use tokio_io::AsyncWrite;

use crate::{CountWrite, Result};

/// Wrapper for `tokio_io::AsyncWrite`, used in the `tokio` family
///
/// *Only available with the `"tokio"` feature*
impl<W: AsyncWrite> AsyncWrite for CountWrite<W> {
    fn poll_write(self: Pin<&mut Self>, ctx: &mut Context, buf: &[u8]) -> Poll<Result<usize>> {
        let Self { inner, count } = unsafe { self.get_unchecked_mut() };
        let pin = unsafe { Pin::new_unchecked(inner) };
        let ret = pin.poll_write(ctx, buf);
        if let Poll::Ready(ret) = &ret {
            if let Ok(written) = &ret {
                *count += *written as u64;
            }
        }
        ret
    }

    fn poll_flush(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Result> {
        unsafe { self.map_unchecked_mut(|cw| &mut cw.inner) }.poll_flush(ctx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Result> {
        unsafe { self.map_unchecked_mut(|cw| &mut cw.inner) }.poll_shutdown(ctx)
    }
}
