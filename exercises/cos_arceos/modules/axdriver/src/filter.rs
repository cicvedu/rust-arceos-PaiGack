use driver_net::{
    BaseDriverOps, DevError, DevResult, DeviceType, EthernetAddress, NetBufPtr, NetDriverOps,
};

pub struct NetFilter<T: BaseDriverOps + NetDriverOps> {
    pub inner: T,
}

impl<T: BaseDriverOps + NetDriverOps> NetFilter<T> {
    pub fn device_name(&self) -> &str {
        self.inner.device_name()
    }

    pub fn device_type(&self) -> DeviceType {
        self.inner.device_type()
    }

    pub fn mac_address(&self) -> EthernetAddress {
        self.inner.mac_address()
    }

    pub fn can_transmit(&self) -> bool {
        self.inner.can_transmit()
    }

    pub fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> Result<(), DevError> {
        self.inner.recycle_rx_buffer(rx_buf)
    }

    pub fn recycle_tx_buffers(&mut self) -> Result<(), DevError> {
        self.inner.recycle_tx_buffers()
    }

    pub fn alloc_tx_buffer(&mut self, size: usize) -> Result<NetBufPtr, DevError> {
        self.inner.alloc_tx_buffer(size)
    }

    pub fn receive(&mut self) -> Result<NetBufPtr, DevError> {
        match self.inner.receive() {
            Ok(v) => {
                log::warn!("Filter: receive len[{:?}]", v.packet_len());
                Ok(v)
            }
            Err(e) => Err(e),
        }
    }

    pub fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
        log::warn!("Filter: transmit len[{}]", tx_buf.packet_len());
        self.inner.transmit(tx_buf)
    }
}
