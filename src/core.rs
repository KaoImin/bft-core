use crate::{
    algorithm::{Bft, INIT_HEIGHT},
    error::BftError,
    types::*,
    FromCore,
};

use crossbeam_channel::{unbounded, Sender};

/// Result of Bft Core.
pub type Result<T> = ::std::result::Result<T, BftError>;

/// A Bft Core
#[derive(Clone, Debug)]
pub struct Core {
    sender: Sender<BftMsg>,
    height: u64,
}

impl Core {
    /// A function to create a new Bft Core.
    pub fn new<T: FromCore + Send + 'static>(s: T, address: Address) -> Self {
        let (sender, internal_receiver) = unbounded();
        Bft::start(s, internal_receiver, address);
        Core {
            sender,
            height: INIT_HEIGHT,
        }
    }

    /// A function to send BFT message to BFT core.
    pub fn send_bft_msg(&mut self, msg: BftMsg) -> Result<()> {
        match msg {
            BftMsg::Status(s) => {
                let status_height = s.height;
                if self.sender.send(BftMsg::Status(s)).is_ok() {
                    if self.height <= status_height {
                        self.height = status_height + 1;
                    }
                    Ok(())
                } else {
                    Err(BftError::SendMsgErr)
                }
            }
            _ => self.sender.send(msg).map_err(|_| BftError::SendMsgErr),
        }
    }

    /// A function to get Bft machine height.
    pub fn get_height(&self) -> u64 {
        self.height
    }
}

#[cfg(test)]
mod test {
    use super::Core as Bft;
    use crate::{types::*, FromCore};
    use crossbeam_channel::{unbounded, Sender};

    #[derive(Debug)]
    enum Error {
        SendErr,
    }

    struct SendMsg(Sender<BftMsg>);

    impl FromCore for SendMsg {
        type error = Error;

        fn send_msg(&self, msg: BftMsg) -> Result<(), Error> {
            self.0.send(msg).map_err(|_| Error::SendErr)?;
            Ok(())
        }
    }

    impl SendMsg {
        fn new() -> Self {
            let (s, _) = unbounded();
            SendMsg(s)
        }
    }

    fn create_status(height: u64) -> BftMsg {
        BftMsg::Status(Status {
            height,
            interval: None,
            authority_list: vec![],
        })
    }

    #[test]
    fn test_height_change() {
        let height: Vec<(u64, u64)> = vec![(1, 2), (2, 3), (1, 3), (4, 5), (6, 7), (5, 7)];
        let mut bft = Bft::new(SendMsg::new(), vec![1]);
        assert_eq!(bft.get_height(), 0);

        for h in height.into_iter() {
            if let Ok(_) = bft.send_bft_msg(create_status(h.0)) {
                assert_eq!(bft.get_height(), h.1);
            } else {
                panic!("Send Error!");
            }
        }
    }
}
