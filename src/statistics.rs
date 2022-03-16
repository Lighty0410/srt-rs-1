use super::Result;
use crate::error::handle_result;
use libsrt_sys as srt;

pub trait Empty<T> {
    fn empty() -> T;
}

impl Empty<srt::CBytePerfMon> for srt::CBytePerfMon {
    fn empty() -> srt::CBytePerfMon {
        srt::CBytePerfMon {
            msTimeStamp: 0,
            pktSentTotal: 0,
            pktRecvTotal: 0,
            pktSndLossTotal: 0,
            pktRcvLossTotal: 0,
            pktRetransTotal: 0,
            pktSentACKTotal: 0,
            pktRecvACKTotal: 0,
            pktSentNAKTotal: 0,
            pktRecvNAKTotal: 0,
            usSndDurationTotal: 0,
            pktSndDropTotal: 0,
            pktRcvDropTotal: 0,
            pktRcvUndecryptTotal: 0,
            byteSentTotal: 0,
            byteRecvTotal: 0,
            byteRcvLossTotal: 0,
            byteRetransTotal: 0,
            byteSndDropTotal: 0,
            byteRcvDropTotal: 0,
            byteRcvUndecryptTotal: 0,
            pktSent: 0,
            pktRecv: 0,
            pktSndLoss: 0,
            pktRcvLoss: 0,
            pktRetrans: 0,
            pktRcvRetrans: 0,
            pktSentACK: 0,
            pktRecvACK: 0,
            pktSentNAK: 0,
            pktRecvNAK: 0,
            mbpsSendRate: 0.0,
            mbpsRecvRate: 0.0,
            usSndDuration: 0,
            pktReorderDistance: 0,
            pktRcvAvgBelatedTime: 0.0,
            pktRcvBelated: 0,
            pktSndDrop: 0,
            pktRcvDrop: 0,
            pktRcvUndecrypt: 0,
            byteSent: 0,
            byteRecv: 0,
            byteRcvLoss: 0,
            byteRetrans: 0,
            byteSndDrop: 0,
            byteRcvDrop: 0,
            byteRcvUndecrypt: 0,
            usPktSndPeriod: 0.0,
            pktFlowWindow: 0,
            pktCongestionWindow: 0,
            pktFlightSize: 0,
            msRTT: 0.0,
            mbpsBandwidth: 0.0,
            byteAvailSndBuf: 0,
            byteAvailRcvBuf: 0,
            mbpsMaxBW: 0.0,
            byteMSS: 0,
            pktSndBuf: 0,
            byteSndBuf: 0,
            msSndBuf: 0,
            msSndTsbPdDelay: 0,
            pktRcvBuf: 0,
            byteRcvBuf: 0,
            msRcvBuf: 0,
            msRcvTsbPdDelay: 0,
            pktSndFilterExtraTotal: 0,
            pktRcvFilterExtraTotal: 0,
            pktRcvFilterSupplyTotal: 0,
            pktRcvFilterLossTotal: 0,
            pktSndFilterExtra: 0,
            pktRcvFilterExtra: 0,
            pktRcvFilterSupply: 0,
            pktRcvFilterLoss: 0,
            pktReorderTolerance: 0,
            pktSentUniqueTotal: 0,
            pktRecvUniqueTotal: 0,
            byteSentUniqueTotal: 0,
            byteRecvUniqueTotal: 0,
            pktSentUnique: 0,
            pktRecvUnique: 0,
            byteSentUnique: 0,
            byteRecvUnique: 0,
        }
    }
}

pub struct Statistics {
    pub statistics: srt::CBytePerfMon,
    id: i32,
}

impl Statistics {
    #[must_use]
    pub fn new(socket_id: i32) -> Statistics {
        Statistics {
            id: socket_id,
            statistics: srt::CBytePerfMon::empty(),
        }
    }
    /// # Errors
    ///
    /// Will return an error if statistics update doesn't succeed.
    pub fn set(&mut self) -> Result<()> {
        let res = unsafe { srt::srt_bstats(self.id, &mut self.statistics, 0) };
        if res != 0 {
            return handle_result((), res);
        }

        Ok(())
    }
}
