#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Device Functional Address"]
    pub faddr: FADDR,
    #[doc = "0x01 - USB Power"]
    pub power: POWER,
    #[doc = "0x02 - USB Transmit Interrupt Status"]
    pub txis: TXIS,
    #[doc = "0x04 - USB Receive Interrupt Status"]
    pub rxis: RXIS,
    #[doc = "0x06 - USB Transmit Interrupt Enable"]
    pub txie: TXIE,
    #[doc = "0x08 - USB Receive Interrupt Enable"]
    pub rxie: RXIE,
    _reserved_6_is: [u8; 0x01],
    _reserved_7_ie: [u8; 0x01],
    #[doc = "0x0c - USB Frame Value"]
    pub frame: FRAME,
    #[doc = "0x0e - USB Endpoint Index"]
    pub epidx: EPIDX,
    #[doc = "0x0f - USB Test Mode"]
    pub test: TEST,
    _reserved11: [u8; 0x10],
    #[doc = "0x20 - USB FIFO Endpoint 0"]
    pub fifo0: FIFO0,
    #[doc = "0x24 - USB FIFO Endpoint 1"]
    pub fifo1: FIFO1,
    #[doc = "0x28 - USB FIFO Endpoint 2"]
    pub fifo2: FIFO2,
    #[doc = "0x2c - USB FIFO Endpoint 3"]
    pub fifo3: FIFO3,
    #[doc = "0x30 - USB FIFO Endpoint 4"]
    pub fifo4: FIFO4,
    #[doc = "0x34 - USB FIFO Endpoint 5"]
    pub fifo5: FIFO5,
    #[doc = "0x38 - USB FIFO Endpoint 6"]
    pub fifo6: FIFO6,
    #[doc = "0x3c - USB FIFO Endpoint 7"]
    pub fifo7: FIFO7,
    _reserved19: [u8; 0x20],
    #[doc = "0x60 - USB Device Control"]
    pub devctl: DEVCTL,
    _reserved20: [u8; 0x01],
    #[doc = "0x62 - USB Transmit Dynamic FIFO Sizing"]
    pub txfifosz: TXFIFOSZ,
    #[doc = "0x63 - USB Receive Dynamic FIFO Sizing"]
    pub rxfifosz: RXFIFOSZ,
    #[doc = "0x64 - USB Transmit FIFO Start Address"]
    pub txfifoadd: TXFIFOADD,
    #[doc = "0x66 - USB Receive FIFO Start Address"]
    pub rxfifoadd: RXFIFOADD,
    _reserved24: [u8; 0x12],
    #[doc = "0x7a - USB Connect Timing"]
    pub contim: CONTIM,
    #[doc = "0x7b - USB OTG VBUS Pulse Timing"]
    pub vplen: VPLEN,
    _reserved26: [u8; 0x01],
    #[doc = "0x7d - USB Full-Speed Last Transaction to End of Frame Timing"]
    pub fseof: FSEOF,
    #[doc = "0x7e - USB Low-Speed Last Transaction to End of Frame Timing"]
    pub lseof: LSEOF,
    _reserved28: [u8; 0x01],
    #[doc = "0x80 - USB Transmit Functional Address Endpoint 0"]
    pub txfuncaddr0: TXFUNCADDR0,
    _reserved29: [u8; 0x01],
    #[doc = "0x82 - USB Transmit Hub Address Endpoint 0"]
    pub txhubaddr0: TXHUBADDR0,
    #[doc = "0x83 - USB Transmit Hub Port Endpoint 0"]
    pub txhubport0: TXHUBPORT0,
    _reserved31: [u8; 0x04],
    #[doc = "0x88 - USB Transmit Functional Address Endpoint 1"]
    pub txfuncaddr1: TXFUNCADDR1,
    _reserved32: [u8; 0x01],
    #[doc = "0x8a - USB Transmit Hub Address Endpoint 1"]
    pub txhubaddr1: TXHUBADDR1,
    #[doc = "0x8b - USB Transmit Hub Port Endpoint 1"]
    pub txhubport1: TXHUBPORT1,
    #[doc = "0x8c - USB Receive Functional Address Endpoint 1"]
    pub rxfuncaddr1: RXFUNCADDR1,
    _reserved35: [u8; 0x01],
    #[doc = "0x8e - USB Receive Hub Address Endpoint 1"]
    pub rxhubaddr1: RXHUBADDR1,
    #[doc = "0x8f - USB Receive Hub Port Endpoint 1"]
    pub rxhubport1: RXHUBPORT1,
    #[doc = "0x90 - USB Transmit Functional Address Endpoint 2"]
    pub txfuncaddr2: TXFUNCADDR2,
    _reserved38: [u8; 0x01],
    #[doc = "0x92 - USB Transmit Hub Address Endpoint 2"]
    pub txhubaddr2: TXHUBADDR2,
    #[doc = "0x93 - USB Transmit Hub Port Endpoint 2"]
    pub txhubport2: TXHUBPORT2,
    #[doc = "0x94 - USB Receive Functional Address Endpoint 2"]
    pub rxfuncaddr2: RXFUNCADDR2,
    _reserved41: [u8; 0x01],
    #[doc = "0x96 - USB Receive Hub Address Endpoint 2"]
    pub rxhubaddr2: RXHUBADDR2,
    #[doc = "0x97 - USB Receive Hub Port Endpoint 2"]
    pub rxhubport2: RXHUBPORT2,
    #[doc = "0x98 - USB Transmit Functional Address Endpoint 3"]
    pub txfuncaddr3: TXFUNCADDR3,
    _reserved44: [u8; 0x01],
    #[doc = "0x9a - USB Transmit Hub Address Endpoint 3"]
    pub txhubaddr3: TXHUBADDR3,
    #[doc = "0x9b - USB Transmit Hub Port Endpoint 3"]
    pub txhubport3: TXHUBPORT3,
    #[doc = "0x9c - USB Receive Functional Address Endpoint 3"]
    pub rxfuncaddr3: RXFUNCADDR3,
    _reserved47: [u8; 0x01],
    #[doc = "0x9e - USB Receive Hub Address Endpoint 3"]
    pub rxhubaddr3: RXHUBADDR3,
    #[doc = "0x9f - USB Receive Hub Port Endpoint 3"]
    pub rxhubport3: RXHUBPORT3,
    #[doc = "0xa0 - USB Transmit Functional Address Endpoint 4"]
    pub txfuncaddr4: TXFUNCADDR4,
    _reserved50: [u8; 0x01],
    #[doc = "0xa2 - USB Transmit Hub Address Endpoint 4"]
    pub txhubaddr4: TXHUBADDR4,
    #[doc = "0xa3 - USB Transmit Hub Port Endpoint 4"]
    pub txhubport4: TXHUBPORT4,
    #[doc = "0xa4 - USB Receive Functional Address Endpoint 4"]
    pub rxfuncaddr4: RXFUNCADDR4,
    _reserved53: [u8; 0x01],
    #[doc = "0xa6 - USB Receive Hub Address Endpoint 4"]
    pub rxhubaddr4: RXHUBADDR4,
    #[doc = "0xa7 - USB Receive Hub Port Endpoint 4"]
    pub rxhubport4: RXHUBPORT4,
    #[doc = "0xa8 - USB Transmit Functional Address Endpoint 5"]
    pub txfuncaddr5: TXFUNCADDR5,
    _reserved56: [u8; 0x01],
    #[doc = "0xaa - USB Transmit Hub Address Endpoint 5"]
    pub txhubaddr5: TXHUBADDR5,
    #[doc = "0xab - USB Transmit Hub Port Endpoint 5"]
    pub txhubport5: TXHUBPORT5,
    #[doc = "0xac - USB Receive Functional Address Endpoint 5"]
    pub rxfuncaddr5: RXFUNCADDR5,
    _reserved59: [u8; 0x01],
    #[doc = "0xae - USB Receive Hub Address Endpoint 5"]
    pub rxhubaddr5: RXHUBADDR5,
    #[doc = "0xaf - USB Receive Hub Port Endpoint 5"]
    pub rxhubport5: RXHUBPORT5,
    #[doc = "0xb0 - USB Transmit Functional Address Endpoint 6"]
    pub txfuncaddr6: TXFUNCADDR6,
    _reserved62: [u8; 0x01],
    #[doc = "0xb2 - USB Transmit Hub Address Endpoint 6"]
    pub txhubaddr6: TXHUBADDR6,
    #[doc = "0xb3 - USB Transmit Hub Port Endpoint 6"]
    pub txhubport6: TXHUBPORT6,
    #[doc = "0xb4 - USB Receive Functional Address Endpoint 6"]
    pub rxfuncaddr6: RXFUNCADDR6,
    _reserved65: [u8; 0x01],
    #[doc = "0xb6 - USB Receive Hub Address Endpoint 6"]
    pub rxhubaddr6: RXHUBADDR6,
    #[doc = "0xb7 - USB Receive Hub Port Endpoint 6"]
    pub rxhubport6: RXHUBPORT6,
    #[doc = "0xb8 - USB Transmit Functional Address Endpoint 7"]
    pub txfuncaddr7: TXFUNCADDR7,
    _reserved68: [u8; 0x01],
    #[doc = "0xba - USB Transmit Hub Address Endpoint 7"]
    pub txhubaddr7: TXHUBADDR7,
    #[doc = "0xbb - USB Transmit Hub Port Endpoint 7"]
    pub txhubport7: TXHUBPORT7,
    #[doc = "0xbc - USB Receive Functional Address Endpoint 7"]
    pub rxfuncaddr7: RXFUNCADDR7,
    _reserved71: [u8; 0x01],
    #[doc = "0xbe - USB Receive Hub Address Endpoint 7"]
    pub rxhubaddr7: RXHUBADDR7,
    #[doc = "0xbf - USB Receive Hub Port Endpoint 7"]
    pub rxhubport7: RXHUBPORT7,
    _reserved73: [u8; 0x42],
    _reserved_73_csrl0: [u8; 0x01],
    #[doc = "0x103 - USB Control and Status Endpoint 0 High"]
    pub csrh0: CSRH0,
    _reserved75: [u8; 0x04],
    #[doc = "0x108 - USB Receive Byte Count Endpoint 0"]
    pub count0: COUNT0,
    _reserved76: [u8; 0x01],
    #[doc = "0x10a - USB Type Endpoint 0"]
    pub type0: TYPE0,
    #[doc = "0x10b - USB NAK Limit"]
    pub naklmt: NAKLMT,
    _reserved78: [u8; 0x04],
    #[doc = "0x110 - USB Maximum Transmit Data Endpoint 1"]
    pub txmaxp1: TXMAXP1,
    _reserved_79_txcsrl1: [u8; 0x01],
    #[doc = "0x113 - USB Transmit Control and Status Endpoint 1 High"]
    pub txcsrh1: TXCSRH1,
    #[doc = "0x114 - USB Maximum Receive Data Endpoint 1"]
    pub rxmaxp1: RXMAXP1,
    _reserved_82_rxcsrl1: [u8; 0x01],
    _reserved_83_rxcsrh1: [u8; 0x01],
    #[doc = "0x118 - USB Receive Byte Count Endpoint 1"]
    pub rxcount1: RXCOUNT1,
    #[doc = "0x11a - USB Host Transmit Configure Type Endpoint 1"]
    pub txtype1: TXTYPE1,
    _reserved_86_txinterval1: [u8; 0x01],
    #[doc = "0x11c - USB Host Configure Receive Type Endpoint 1"]
    pub rxtype1: RXTYPE1,
    _reserved_88_rxinterval1: [u8; 0x01],
    _reserved89: [u8; 0x02],
    #[doc = "0x120 - USB Maximum Transmit Data Endpoint 2"]
    pub txmaxp2: TXMAXP2,
    _reserved_90_txcsrl2: [u8; 0x01],
    #[doc = "0x123 - USB Transmit Control and Status Endpoint 2 High"]
    pub txcsrh2: TXCSRH2,
    #[doc = "0x124 - USB Maximum Receive Data Endpoint 2"]
    pub rxmaxp2: RXMAXP2,
    _reserved_93_rxcsrl2: [u8; 0x01],
    _reserved_94_rxcsrh2: [u8; 0x01],
    #[doc = "0x128 - USB Receive Byte Count Endpoint 2"]
    pub rxcount2: RXCOUNT2,
    #[doc = "0x12a - USB Host Transmit Configure Type Endpoint 2"]
    pub txtype2: TXTYPE2,
    _reserved_97_txinterval2: [u8; 0x01],
    #[doc = "0x12c - USB Host Configure Receive Type Endpoint 2"]
    pub rxtype2: RXTYPE2,
    _reserved_99_rxinterval2: [u8; 0x01],
    _reserved100: [u8; 0x02],
    #[doc = "0x130 - USB Maximum Transmit Data Endpoint 3"]
    pub txmaxp3: TXMAXP3,
    _reserved_101_txcsrl3: [u8; 0x01],
    #[doc = "0x133 - USB Transmit Control and Status Endpoint 3 High"]
    pub txcsrh3: TXCSRH3,
    #[doc = "0x134 - USB Maximum Receive Data Endpoint 3"]
    pub rxmaxp3: RXMAXP3,
    _reserved_104_rxcsrl3: [u8; 0x01],
    _reserved_105_rxcsrh3: [u8; 0x01],
    #[doc = "0x138 - USB Receive Byte Count Endpoint 3"]
    pub rxcount3: RXCOUNT3,
    #[doc = "0x13a - USB Host Transmit Configure Type Endpoint 3"]
    pub txtype3: TXTYPE3,
    _reserved_108_txinterval3: [u8; 0x01],
    #[doc = "0x13c - USB Host Configure Receive Type Endpoint 3"]
    pub rxtype3: RXTYPE3,
    _reserved_110_rxinterval3: [u8; 0x01],
    _reserved111: [u8; 0x02],
    #[doc = "0x140 - USB Maximum Transmit Data Endpoint 4"]
    pub txmaxp4: TXMAXP4,
    _reserved_112_txcsrl4: [u8; 0x01],
    #[doc = "0x143 - USB Transmit Control and Status Endpoint 4 High"]
    pub txcsrh4: TXCSRH4,
    #[doc = "0x144 - USB Maximum Receive Data Endpoint 4"]
    pub rxmaxp4: RXMAXP4,
    _reserved_115_rxcsrl4: [u8; 0x01],
    _reserved_116_rxcsrh4: [u8; 0x01],
    #[doc = "0x148 - USB Receive Byte Count Endpoint 4"]
    pub rxcount4: RXCOUNT4,
    #[doc = "0x14a - USB Host Transmit Configure Type Endpoint 4"]
    pub txtype4: TXTYPE4,
    _reserved_119_txinterval4: [u8; 0x01],
    #[doc = "0x14c - USB Host Configure Receive Type Endpoint 4"]
    pub rxtype4: RXTYPE4,
    _reserved_121_rxinterval4: [u8; 0x01],
    _reserved122: [u8; 0x02],
    #[doc = "0x150 - USB Maximum Transmit Data Endpoint 5"]
    pub txmaxp5: TXMAXP5,
    _reserved_123_txcsrl5: [u8; 0x01],
    #[doc = "0x153 - USB Transmit Control and Status Endpoint 5 High"]
    pub txcsrh5: TXCSRH5,
    #[doc = "0x154 - USB Maximum Receive Data Endpoint 5"]
    pub rxmaxp5: RXMAXP5,
    _reserved_126_rxcsrl5: [u8; 0x01],
    _reserved_127_rxcsrh5: [u8; 0x01],
    #[doc = "0x158 - USB Receive Byte Count Endpoint 5"]
    pub rxcount5: RXCOUNT5,
    #[doc = "0x15a - USB Host Transmit Configure Type Endpoint 5"]
    pub txtype5: TXTYPE5,
    _reserved_130_txinterval5: [u8; 0x01],
    #[doc = "0x15c - USB Host Configure Receive Type Endpoint 5"]
    pub rxtype5: RXTYPE5,
    _reserved_132_rxinterval5: [u8; 0x01],
    _reserved133: [u8; 0x02],
    #[doc = "0x160 - USB Maximum Transmit Data Endpoint 6"]
    pub txmaxp6: TXMAXP6,
    _reserved_134_txcsrl6: [u8; 0x01],
    #[doc = "0x163 - USB Transmit Control and Status Endpoint 6 High"]
    pub txcsrh6: TXCSRH6,
    #[doc = "0x164 - USB Maximum Receive Data Endpoint 6"]
    pub rxmaxp6: RXMAXP6,
    _reserved_137_rxcsrl6: [u8; 0x01],
    _reserved_138_rxcsrh6: [u8; 0x01],
    #[doc = "0x168 - USB Receive Byte Count Endpoint 6"]
    pub rxcount6: RXCOUNT6,
    #[doc = "0x16a - USB Host Transmit Configure Type Endpoint 6"]
    pub txtype6: TXTYPE6,
    _reserved_141_txinterval6: [u8; 0x01],
    #[doc = "0x16c - USB Host Configure Receive Type Endpoint 6"]
    pub rxtype6: RXTYPE6,
    _reserved_143_rxinterval6: [u8; 0x01],
    _reserved144: [u8; 0x02],
    #[doc = "0x170 - USB Maximum Transmit Data Endpoint 7"]
    pub txmaxp7: TXMAXP7,
    _reserved_145_txcsrl7: [u8; 0x01],
    #[doc = "0x173 - USB Transmit Control and Status Endpoint 7 High"]
    pub txcsrh7: TXCSRH7,
    #[doc = "0x174 - USB Maximum Receive Data Endpoint 7"]
    pub rxmaxp7: RXMAXP7,
    _reserved_148_rxcsrl7: [u8; 0x01],
    _reserved_149_rxcsrh7: [u8; 0x01],
    #[doc = "0x178 - USB Receive Byte Count Endpoint 7"]
    pub rxcount7: RXCOUNT7,
    #[doc = "0x17a - USB Host Transmit Configure Type Endpoint 7"]
    pub txtype7: TXTYPE7,
    _reserved_152_txinterval7: [u8; 0x01],
    #[doc = "0x17c - USB Host Configure Receive Type Endpoint 7"]
    pub rxtype7: RXTYPE7,
    _reserved_154_rxinterval7: [u8; 0x01],
    _reserved155: [u8; 0x0186],
    #[doc = "0x304 - USB Request Packet Count in Block Transfer Endpoint 1"]
    pub rqpktcount1: RQPKTCOUNT1,
    _reserved156: [u8; 0x02],
    #[doc = "0x308 - USB Request Packet Count in Block Transfer Endpoint 2"]
    pub rqpktcount2: RQPKTCOUNT2,
    _reserved157: [u8; 0x02],
    #[doc = "0x30c - USB Request Packet Count in Block Transfer Endpoint 3"]
    pub rqpktcount3: RQPKTCOUNT3,
    _reserved158: [u8; 0x02],
    #[doc = "0x310 - USB Request Packet Count in Block Transfer Endpoint 4"]
    pub rqpktcount4: RQPKTCOUNT4,
    _reserved159: [u8; 0x02],
    #[doc = "0x314 - USB Request Packet Count in Block Transfer Endpoint 5"]
    pub rqpktcount5: RQPKTCOUNT5,
    _reserved160: [u8; 0x02],
    #[doc = "0x318 - USB Request Packet Count in Block Transfer Endpoint 6"]
    pub rqpktcount6: RQPKTCOUNT6,
    _reserved161: [u8; 0x02],
    #[doc = "0x31c - USB Request Packet Count in Block Transfer Endpoint 7"]
    pub rqpktcount7: RQPKTCOUNT7,
    _reserved162: [u8; 0x22],
    #[doc = "0x340 - USB Receive Double Packet Buffer Disable"]
    pub rxdpktbufdis: RXDPKTBUFDIS,
    #[doc = "0x342 - USB Transmit Double Packet Buffer Disable"]
    pub txdpktbufdis: TXDPKTBUFDIS,
    _reserved164: [u8; 0xbc],
    #[doc = "0x400 - USB External Power Control"]
    pub epc: EPC,
    #[doc = "0x404 - USB External Power Control Raw Interrupt Status"]
    pub epcris: EPCRIS,
    #[doc = "0x408 - USB External Power Control Interrupt Mask"]
    pub epcim: EPCIM,
    #[doc = "0x40c - USB External Power Control Interrupt Status and Clear"]
    pub epcisc: EPCISC,
    #[doc = "0x410 - USB Device RESUME Raw Interrupt Status"]
    pub drris: DRRIS,
    #[doc = "0x414 - USB Device RESUME Interrupt Mask"]
    pub drim: DRIM,
    #[doc = "0x418 - USB Device RESUME Interrupt Status and Clear"]
    pub drisc: DRISC,
    #[doc = "0x41c - USB General-Purpose Control and Status"]
    pub gpcs: GPCS,
    _reserved172: [u8; 0x10],
    #[doc = "0x430 - USB VBUS Droop Control"]
    pub vdc: VDC,
    #[doc = "0x434 - USB VBUS Droop Control Raw Interrupt Status"]
    pub vdcris: VDCRIS,
    #[doc = "0x438 - USB VBUS Droop Control Interrupt Mask"]
    pub vdcim: VDCIM,
    #[doc = "0x43c - USB VBUS Droop Control Interrupt Status and Clear"]
    pub vdcisc: VDCISC,
    _reserved176: [u8; 0x04],
    #[doc = "0x444 - USB ID Valid Detect Raw Interrupt Status"]
    pub idvris: IDVRIS,
    #[doc = "0x448 - USB ID Valid Detect Interrupt Mask"]
    pub idvim: IDVIM,
    #[doc = "0x44c - USB ID Valid Detect Interrupt Status and Clear"]
    pub idvisc: IDVISC,
    #[doc = "0x450 - USB DMA Select"]
    pub dmasel: DMASEL,
    _reserved180: [u8; 0x0b6c],
    #[doc = "0xfc0 - USB Peripheral Properties"]
    pub pp: PP,
}
impl RegisterBlock {
    #[doc = "0x0a - USB General Interrupt Status"]
    #[inline(always)]
    pub const fn usb0_alt_is(&self) -> &USB0_ALT_IS {
        unsafe { &*(self as *const Self).cast::<u8>().add(10usize).cast() }
    }
    #[doc = "0x0a - USB General Interrupt Status"]
    #[inline(always)]
    pub const fn is(&self) -> &IS {
        unsafe { &*(self as *const Self).cast::<u8>().add(10usize).cast() }
    }
    #[doc = "0x0b - USB Interrupt Enable"]
    #[inline(always)]
    pub const fn usb0_alt_ie(&self) -> &USB0_ALT_IE {
        unsafe { &*(self as *const Self).cast::<u8>().add(11usize).cast() }
    }
    #[doc = "0x0b - USB Interrupt Enable"]
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        unsafe { &*(self as *const Self).cast::<u8>().add(11usize).cast() }
    }
    #[doc = "0x102 - USB Control and Status Endpoint 0 Low"]
    #[inline(always)]
    pub const fn usb0_alt_csrl0(&self) -> &USB0_ALT_CSRL0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(258usize).cast() }
    }
    #[doc = "0x102 - USB Control and Status Endpoint 0 Low"]
    #[inline(always)]
    pub const fn csrl0(&self) -> &CSRL0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(258usize).cast() }
    }
    #[doc = "0x112 - USB Transmit Control and Status Endpoint 1 Low"]
    #[inline(always)]
    pub const fn usb0_alt_txcsrl1(&self) -> &USB0_ALT_TXCSRL1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(274usize).cast() }
    }
    #[doc = "0x112 - USB Transmit Control and Status Endpoint 1 Low"]
    #[inline(always)]
    pub const fn txcsrl1(&self) -> &TXCSRL1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(274usize).cast() }
    }
    #[doc = "0x116 - USB Receive Control and Status Endpoint 1 Low"]
    #[inline(always)]
    pub const fn usb0_alt_rxcsrl1(&self) -> &USB0_ALT_RXCSRL1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(278usize).cast() }
    }
    #[doc = "0x116 - USB Receive Control and Status Endpoint 1 Low"]
    #[inline(always)]
    pub const fn rxcsrl1(&self) -> &RXCSRL1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(278usize).cast() }
    }
    #[doc = "0x117 - USB Receive Control and Status Endpoint 1 High"]
    #[inline(always)]
    pub const fn usb0_alt_rxcsrh1(&self) -> &USB0_ALT_RXCSRH1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(279usize).cast() }
    }
    #[doc = "0x117 - USB Receive Control and Status Endpoint 1 High"]
    #[inline(always)]
    pub const fn rxcsrh1(&self) -> &RXCSRH1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(279usize).cast() }
    }
    #[doc = "0x11b - USB Host Transmit Interval Endpoint 1"]
    #[inline(always)]
    pub const fn usb0_alt_txinterval1(&self) -> &USB0_ALT_TXINTERVAL1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(283usize).cast() }
    }
    #[doc = "0x11b - USB Host Transmit Interval Endpoint 1"]
    #[inline(always)]
    pub const fn txinterval1(&self) -> &TXINTERVAL1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(283usize).cast() }
    }
    #[doc = "0x11d - USB Host Receive Polling Interval Endpoint 1"]
    #[inline(always)]
    pub const fn usb0_alt_rxinterval1(&self) -> &USB0_ALT_RXINTERVAL1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(285usize).cast() }
    }
    #[doc = "0x11d - USB Host Receive Polling Interval Endpoint 1"]
    #[inline(always)]
    pub const fn rxinterval1(&self) -> &RXINTERVAL1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(285usize).cast() }
    }
    #[doc = "0x122 - USB Transmit Control and Status Endpoint 2 Low"]
    #[inline(always)]
    pub const fn usb0_alt_txcsrl2(&self) -> &USB0_ALT_TXCSRL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(290usize).cast() }
    }
    #[doc = "0x122 - USB Transmit Control and Status Endpoint 2 Low"]
    #[inline(always)]
    pub const fn txcsrl2(&self) -> &TXCSRL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(290usize).cast() }
    }
    #[doc = "0x126 - USB Receive Control and Status Endpoint 2 Low"]
    #[inline(always)]
    pub const fn usb0_alt_rxcsrl2(&self) -> &USB0_ALT_RXCSRL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(294usize).cast() }
    }
    #[doc = "0x126 - USB Receive Control and Status Endpoint 2 Low"]
    #[inline(always)]
    pub const fn rxcsrl2(&self) -> &RXCSRL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(294usize).cast() }
    }
    #[doc = "0x127 - USB Receive Control and Status Endpoint 2 High"]
    #[inline(always)]
    pub const fn usb0_alt_rxcsrh2(&self) -> &USB0_ALT_RXCSRH2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(295usize).cast() }
    }
    #[doc = "0x127 - USB Receive Control and Status Endpoint 2 High"]
    #[inline(always)]
    pub const fn rxcsrh2(&self) -> &RXCSRH2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(295usize).cast() }
    }
    #[doc = "0x12b - USB Host Transmit Interval Endpoint 2"]
    #[inline(always)]
    pub const fn usb0_alt_txinterval2(&self) -> &USB0_ALT_TXINTERVAL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(299usize).cast() }
    }
    #[doc = "0x12b - USB Host Transmit Interval Endpoint 2"]
    #[inline(always)]
    pub const fn txinterval2(&self) -> &TXINTERVAL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(299usize).cast() }
    }
    #[doc = "0x12d - USB Host Receive Polling Interval Endpoint 2"]
    #[inline(always)]
    pub const fn usb0_alt_rxinterval2(&self) -> &USB0_ALT_RXINTERVAL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(301usize).cast() }
    }
    #[doc = "0x12d - USB Host Receive Polling Interval Endpoint 2"]
    #[inline(always)]
    pub const fn rxinterval2(&self) -> &RXINTERVAL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(301usize).cast() }
    }
    #[doc = "0x132 - USB Transmit Control and Status Endpoint 3 Low"]
    #[inline(always)]
    pub const fn usb0_alt_txcsrl3(&self) -> &USB0_ALT_TXCSRL3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(306usize).cast() }
    }
    #[doc = "0x132 - USB Transmit Control and Status Endpoint 3 Low"]
    #[inline(always)]
    pub const fn txcsrl3(&self) -> &TXCSRL3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(306usize).cast() }
    }
    #[doc = "0x136 - USB Receive Control and Status Endpoint 3 Low"]
    #[inline(always)]
    pub const fn usb0_alt_rxcsrl3(&self) -> &USB0_ALT_RXCSRL3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(310usize).cast() }
    }
    #[doc = "0x136 - USB Receive Control and Status Endpoint 3 Low"]
    #[inline(always)]
    pub const fn rxcsrl3(&self) -> &RXCSRL3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(310usize).cast() }
    }
    #[doc = "0x137 - USB Receive Control and Status Endpoint 3 High"]
    #[inline(always)]
    pub const fn usb0_alt_rxcsrh3(&self) -> &USB0_ALT_RXCSRH3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(311usize).cast() }
    }
    #[doc = "0x137 - USB Receive Control and Status Endpoint 3 High"]
    #[inline(always)]
    pub const fn rxcsrh3(&self) -> &RXCSRH3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(311usize).cast() }
    }
    #[doc = "0x13b - USB Host Transmit Interval Endpoint 3"]
    #[inline(always)]
    pub const fn usb0_alt_txinterval3(&self) -> &USB0_ALT_TXINTERVAL3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(315usize).cast() }
    }
    #[doc = "0x13b - USB Host Transmit Interval Endpoint 3"]
    #[inline(always)]
    pub const fn txinterval3(&self) -> &TXINTERVAL3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(315usize).cast() }
    }
    #[doc = "0x13d - USB Host Receive Polling Interval Endpoint 3"]
    #[inline(always)]
    pub const fn usb0_alt_rxinterval3(&self) -> &USB0_ALT_RXINTERVAL3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(317usize).cast() }
    }
    #[doc = "0x13d - USB Host Receive Polling Interval Endpoint 3"]
    #[inline(always)]
    pub const fn rxinterval3(&self) -> &RXINTERVAL3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(317usize).cast() }
    }
    #[doc = "0x142 - USB Transmit Control and Status Endpoint 4 Low"]
    #[inline(always)]
    pub const fn usb0_alt_txcsrl4(&self) -> &USB0_ALT_TXCSRL4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(322usize).cast() }
    }
    #[doc = "0x142 - USB Transmit Control and Status Endpoint 4 Low"]
    #[inline(always)]
    pub const fn txcsrl4(&self) -> &TXCSRL4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(322usize).cast() }
    }
    #[doc = "0x146 - USB Receive Control and Status Endpoint 4 Low"]
    #[inline(always)]
    pub const fn usb0_alt_rxcsrl4(&self) -> &USB0_ALT_RXCSRL4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(326usize).cast() }
    }
    #[doc = "0x146 - USB Receive Control and Status Endpoint 4 Low"]
    #[inline(always)]
    pub const fn rxcsrl4(&self) -> &RXCSRL4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(326usize).cast() }
    }
    #[doc = "0x147 - USB Receive Control and Status Endpoint 4 High"]
    #[inline(always)]
    pub const fn usb0_alt_rxcsrh4(&self) -> &USB0_ALT_RXCSRH4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(327usize).cast() }
    }
    #[doc = "0x147 - USB Receive Control and Status Endpoint 4 High"]
    #[inline(always)]
    pub const fn rxcsrh4(&self) -> &RXCSRH4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(327usize).cast() }
    }
    #[doc = "0x14b - USB Host Transmit Interval Endpoint 4"]
    #[inline(always)]
    pub const fn usb0_alt_txinterval4(&self) -> &USB0_ALT_TXINTERVAL4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(331usize).cast() }
    }
    #[doc = "0x14b - USB Host Transmit Interval Endpoint 4"]
    #[inline(always)]
    pub const fn txinterval4(&self) -> &TXINTERVAL4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(331usize).cast() }
    }
    #[doc = "0x14d - USB Host Receive Polling Interval Endpoint 4"]
    #[inline(always)]
    pub const fn usb0_alt_rxinterval4(&self) -> &USB0_ALT_RXINTERVAL4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(333usize).cast() }
    }
    #[doc = "0x14d - USB Host Receive Polling Interval Endpoint 4"]
    #[inline(always)]
    pub const fn rxinterval4(&self) -> &RXINTERVAL4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(333usize).cast() }
    }
    #[doc = "0x152 - USB Transmit Control and Status Endpoint 5 Low"]
    #[inline(always)]
    pub const fn usb0_alt_txcsrl5(&self) -> &USB0_ALT_TXCSRL5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(338usize).cast() }
    }
    #[doc = "0x152 - USB Transmit Control and Status Endpoint 5 Low"]
    #[inline(always)]
    pub const fn txcsrl5(&self) -> &TXCSRL5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(338usize).cast() }
    }
    #[doc = "0x156 - USB Receive Control and Status Endpoint 5 Low"]
    #[inline(always)]
    pub const fn usb0_alt_rxcsrl5(&self) -> &USB0_ALT_RXCSRL5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(342usize).cast() }
    }
    #[doc = "0x156 - USB Receive Control and Status Endpoint 5 Low"]
    #[inline(always)]
    pub const fn rxcsrl5(&self) -> &RXCSRL5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(342usize).cast() }
    }
    #[doc = "0x157 - USB Receive Control and Status Endpoint 5 High"]
    #[inline(always)]
    pub const fn usb0_alt_rxcsrh5(&self) -> &USB0_ALT_RXCSRH5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(343usize).cast() }
    }
    #[doc = "0x157 - USB Receive Control and Status Endpoint 5 High"]
    #[inline(always)]
    pub const fn rxcsrh5(&self) -> &RXCSRH5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(343usize).cast() }
    }
    #[doc = "0x15b - USB Host Transmit Interval Endpoint 5"]
    #[inline(always)]
    pub const fn usb0_alt_txinterval5(&self) -> &USB0_ALT_TXINTERVAL5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(347usize).cast() }
    }
    #[doc = "0x15b - USB Host Transmit Interval Endpoint 5"]
    #[inline(always)]
    pub const fn txinterval5(&self) -> &TXINTERVAL5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(347usize).cast() }
    }
    #[doc = "0x15d - USB Host Receive Polling Interval Endpoint 5"]
    #[inline(always)]
    pub const fn usb0_alt_rxinterval5(&self) -> &USB0_ALT_RXINTERVAL5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(349usize).cast() }
    }
    #[doc = "0x15d - USB Host Receive Polling Interval Endpoint 5"]
    #[inline(always)]
    pub const fn rxinterval5(&self) -> &RXINTERVAL5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(349usize).cast() }
    }
    #[doc = "0x162 - USB Transmit Control and Status Endpoint 6 Low"]
    #[inline(always)]
    pub const fn usb0_alt_txcsrl6(&self) -> &USB0_ALT_TXCSRL6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(354usize).cast() }
    }
    #[doc = "0x162 - USB Transmit Control and Status Endpoint 6 Low"]
    #[inline(always)]
    pub const fn txcsrl6(&self) -> &TXCSRL6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(354usize).cast() }
    }
    #[doc = "0x166 - USB Receive Control and Status Endpoint 6 Low"]
    #[inline(always)]
    pub const fn usb0_alt_rxcsrl6(&self) -> &USB0_ALT_RXCSRL6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(358usize).cast() }
    }
    #[doc = "0x166 - USB Receive Control and Status Endpoint 6 Low"]
    #[inline(always)]
    pub const fn rxcsrl6(&self) -> &RXCSRL6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(358usize).cast() }
    }
    #[doc = "0x167 - USB Receive Control and Status Endpoint 6 High"]
    #[inline(always)]
    pub const fn usb0_alt_rxcsrh6(&self) -> &USB0_ALT_RXCSRH6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(359usize).cast() }
    }
    #[doc = "0x167 - USB Receive Control and Status Endpoint 6 High"]
    #[inline(always)]
    pub const fn rxcsrh6(&self) -> &RXCSRH6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(359usize).cast() }
    }
    #[doc = "0x16b - USB Host Transmit Interval Endpoint 6"]
    #[inline(always)]
    pub const fn usb0_alt_txinterval6(&self) -> &USB0_ALT_TXINTERVAL6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(363usize).cast() }
    }
    #[doc = "0x16b - USB Host Transmit Interval Endpoint 6"]
    #[inline(always)]
    pub const fn txinterval6(&self) -> &TXINTERVAL6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(363usize).cast() }
    }
    #[doc = "0x16d - USB Host Receive Polling Interval Endpoint 6"]
    #[inline(always)]
    pub const fn usb0_alt_rxinterval6(&self) -> &USB0_ALT_RXINTERVAL6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(365usize).cast() }
    }
    #[doc = "0x16d - USB Host Receive Polling Interval Endpoint 6"]
    #[inline(always)]
    pub const fn rxinterval6(&self) -> &RXINTERVAL6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(365usize).cast() }
    }
    #[doc = "0x172 - USB Transmit Control and Status Endpoint 7 Low"]
    #[inline(always)]
    pub const fn usb0_alt_txcsrl7(&self) -> &USB0_ALT_TXCSRL7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(370usize).cast() }
    }
    #[doc = "0x172 - USB Transmit Control and Status Endpoint 7 Low"]
    #[inline(always)]
    pub const fn txcsrl7(&self) -> &TXCSRL7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(370usize).cast() }
    }
    #[doc = "0x176 - USB Receive Control and Status Endpoint 7 Low"]
    #[inline(always)]
    pub const fn usb0_alt_rxcsrl7(&self) -> &USB0_ALT_RXCSRL7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(374usize).cast() }
    }
    #[doc = "0x176 - USB Receive Control and Status Endpoint 7 Low"]
    #[inline(always)]
    pub const fn rxcsrl7(&self) -> &RXCSRL7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(374usize).cast() }
    }
    #[doc = "0x177 - USB Receive Control and Status Endpoint 7 High"]
    #[inline(always)]
    pub const fn usb0_alt_rxcsrh7(&self) -> &USB0_ALT_RXCSRH7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(375usize).cast() }
    }
    #[doc = "0x177 - USB Receive Control and Status Endpoint 7 High"]
    #[inline(always)]
    pub const fn rxcsrh7(&self) -> &RXCSRH7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(375usize).cast() }
    }
    #[doc = "0x17b - USB Host Transmit Interval Endpoint 7"]
    #[inline(always)]
    pub const fn usb0_alt_txinterval7(&self) -> &USB0_ALT_TXINTERVAL7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(379usize).cast() }
    }
    #[doc = "0x17b - USB Host Transmit Interval Endpoint 7"]
    #[inline(always)]
    pub const fn txinterval7(&self) -> &TXINTERVAL7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(379usize).cast() }
    }
    #[doc = "0x17d - USB Host Receive Polling Interval Endpoint 7"]
    #[inline(always)]
    pub const fn usb0_alt_rxinterval7(&self) -> &USB0_ALT_RXINTERVAL7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(381usize).cast() }
    }
    #[doc = "0x17d - USB Host Receive Polling Interval Endpoint 7"]
    #[inline(always)]
    pub const fn rxinterval7(&self) -> &RXINTERVAL7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(381usize).cast() }
    }
}
#[doc = "FADDR (rw) register accessor: USB Device Functional Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`faddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`faddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@faddr`]
module"]
pub type FADDR = crate::Reg<faddr::FADDR_SPEC>;
#[doc = "USB Device Functional Address"]
pub mod faddr;
#[doc = "POWER (rw) register accessor: USB Power\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`]
module"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "USB Power"]
pub mod power;
#[doc = "TXIS (rw) register accessor: USB Transmit Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txis`]
module"]
pub type TXIS = crate::Reg<txis::TXIS_SPEC>;
#[doc = "USB Transmit Interrupt Status"]
pub mod txis;
#[doc = "RXIS (rw) register accessor: USB Receive Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxis`]
module"]
pub type RXIS = crate::Reg<rxis::RXIS_SPEC>;
#[doc = "USB Receive Interrupt Status"]
pub mod rxis;
#[doc = "TXIE (rw) register accessor: USB Transmit Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txie`]
module"]
pub type TXIE = crate::Reg<txie::TXIE_SPEC>;
#[doc = "USB Transmit Interrupt Enable"]
pub mod txie;
#[doc = "RXIE (rw) register accessor: USB Receive Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxie`]
module"]
pub type RXIE = crate::Reg<rxie::RXIE_SPEC>;
#[doc = "USB Receive Interrupt Enable"]
pub mod rxie;
#[doc = "IS (rw) register accessor: USB General Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`is::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is`]
module"]
pub type IS = crate::Reg<is::IS_SPEC>;
#[doc = "USB General Interrupt Status"]
pub mod is;
#[doc = "USB0_ALT_IS (rw) register accessor: USB General Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_is::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_is::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_is`]
module"]
pub type USB0_ALT_IS = crate::Reg<usb0_alt_is::USB0_ALT_IS_SPEC>;
#[doc = "USB General Interrupt Status"]
pub mod usb0_alt_is;
#[doc = "IE (rw) register accessor: USB Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "USB Interrupt Enable"]
pub mod ie;
#[doc = "USB0_ALT_IE (rw) register accessor: USB Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_ie`]
module"]
pub type USB0_ALT_IE = crate::Reg<usb0_alt_ie::USB0_ALT_IE_SPEC>;
#[doc = "USB Interrupt Enable"]
pub mod usb0_alt_ie;
#[doc = "FRAME (rw) register accessor: USB Frame Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frame::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frame::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame`]
module"]
pub type FRAME = crate::Reg<frame::FRAME_SPEC>;
#[doc = "USB Frame Value"]
pub mod frame;
#[doc = "EPIDX (rw) register accessor: USB Endpoint Index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epidx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epidx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epidx`]
module"]
pub type EPIDX = crate::Reg<epidx::EPIDX_SPEC>;
#[doc = "USB Endpoint Index"]
pub mod epidx;
#[doc = "TEST (rw) register accessor: USB Test Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`]
module"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "USB Test Mode"]
pub mod test;
#[doc = "FIFO0 (rw) register accessor: USB FIFO Endpoint 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo0`]
module"]
pub type FIFO0 = crate::Reg<fifo0::FIFO0_SPEC>;
#[doc = "USB FIFO Endpoint 0"]
pub mod fifo0;
#[doc = "FIFO1 (rw) register accessor: USB FIFO Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo1`]
module"]
pub type FIFO1 = crate::Reg<fifo1::FIFO1_SPEC>;
#[doc = "USB FIFO Endpoint 1"]
pub mod fifo1;
#[doc = "FIFO2 (rw) register accessor: USB FIFO Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo2`]
module"]
pub type FIFO2 = crate::Reg<fifo2::FIFO2_SPEC>;
#[doc = "USB FIFO Endpoint 2"]
pub mod fifo2;
#[doc = "FIFO3 (rw) register accessor: USB FIFO Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo3`]
module"]
pub type FIFO3 = crate::Reg<fifo3::FIFO3_SPEC>;
#[doc = "USB FIFO Endpoint 3"]
pub mod fifo3;
#[doc = "FIFO4 (rw) register accessor: USB FIFO Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo4`]
module"]
pub type FIFO4 = crate::Reg<fifo4::FIFO4_SPEC>;
#[doc = "USB FIFO Endpoint 4"]
pub mod fifo4;
#[doc = "FIFO5 (rw) register accessor: USB FIFO Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo5`]
module"]
pub type FIFO5 = crate::Reg<fifo5::FIFO5_SPEC>;
#[doc = "USB FIFO Endpoint 5"]
pub mod fifo5;
#[doc = "FIFO6 (rw) register accessor: USB FIFO Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo6`]
module"]
pub type FIFO6 = crate::Reg<fifo6::FIFO6_SPEC>;
#[doc = "USB FIFO Endpoint 6"]
pub mod fifo6;
#[doc = "FIFO7 (rw) register accessor: USB FIFO Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo7`]
module"]
pub type FIFO7 = crate::Reg<fifo7::FIFO7_SPEC>;
#[doc = "USB FIFO Endpoint 7"]
pub mod fifo7;
#[doc = "DEVCTL (rw) register accessor: USB Device Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devctl`]
module"]
pub type DEVCTL = crate::Reg<devctl::DEVCTL_SPEC>;
#[doc = "USB Device Control"]
pub mod devctl;
#[doc = "TXFIFOSZ (rw) register accessor: USB Transmit Dynamic FIFO Sizing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfifosz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfifosz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifosz`]
module"]
pub type TXFIFOSZ = crate::Reg<txfifosz::TXFIFOSZ_SPEC>;
#[doc = "USB Transmit Dynamic FIFO Sizing"]
pub mod txfifosz;
#[doc = "RXFIFOSZ (rw) register accessor: USB Receive Dynamic FIFO Sizing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfifosz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfifosz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifosz`]
module"]
pub type RXFIFOSZ = crate::Reg<rxfifosz::RXFIFOSZ_SPEC>;
#[doc = "USB Receive Dynamic FIFO Sizing"]
pub mod rxfifosz;
#[doc = "TXFIFOADD (rw) register accessor: USB Transmit FIFO Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfifoadd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfifoadd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifoadd`]
module"]
pub type TXFIFOADD = crate::Reg<txfifoadd::TXFIFOADD_SPEC>;
#[doc = "USB Transmit FIFO Start Address"]
pub mod txfifoadd;
#[doc = "RXFIFOADD (rw) register accessor: USB Receive FIFO Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfifoadd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfifoadd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifoadd`]
module"]
pub type RXFIFOADD = crate::Reg<rxfifoadd::RXFIFOADD_SPEC>;
#[doc = "USB Receive FIFO Start Address"]
pub mod rxfifoadd;
#[doc = "CONTIM (rw) register accessor: USB Connect Timing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`contim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`contim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@contim`]
module"]
pub type CONTIM = crate::Reg<contim::CONTIM_SPEC>;
#[doc = "USB Connect Timing"]
pub mod contim;
#[doc = "VPLEN (rw) register accessor: USB OTG VBUS Pulse Timing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vplen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vplen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vplen`]
module"]
pub type VPLEN = crate::Reg<vplen::VPLEN_SPEC>;
#[doc = "USB OTG VBUS Pulse Timing"]
pub mod vplen;
#[doc = "FSEOF (rw) register accessor: USB Full-Speed Last Transaction to End of Frame Timing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fseof::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fseof::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fseof`]
module"]
pub type FSEOF = crate::Reg<fseof::FSEOF_SPEC>;
#[doc = "USB Full-Speed Last Transaction to End of Frame Timing"]
pub mod fseof;
#[doc = "LSEOF (rw) register accessor: USB Low-Speed Last Transaction to End of Frame Timing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lseof::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lseof::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lseof`]
module"]
pub type LSEOF = crate::Reg<lseof::LSEOF_SPEC>;
#[doc = "USB Low-Speed Last Transaction to End of Frame Timing"]
pub mod lseof;
#[doc = "TXFUNCADDR0 (rw) register accessor: USB Transmit Functional Address Endpoint 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfuncaddr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfuncaddr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfuncaddr0`]
module"]
pub type TXFUNCADDR0 = crate::Reg<txfuncaddr0::TXFUNCADDR0_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 0"]
pub mod txfuncaddr0;
#[doc = "TXHUBADDR0 (rw) register accessor: USB Transmit Hub Address Endpoint 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubaddr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubaddr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubaddr0`]
module"]
pub type TXHUBADDR0 = crate::Reg<txhubaddr0::TXHUBADDR0_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 0"]
pub mod txhubaddr0;
#[doc = "TXHUBPORT0 (rw) register accessor: USB Transmit Hub Port Endpoint 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubport0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubport0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubport0`]
module"]
pub type TXHUBPORT0 = crate::Reg<txhubport0::TXHUBPORT0_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 0"]
pub mod txhubport0;
#[doc = "TXFUNCADDR1 (rw) register accessor: USB Transmit Functional Address Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfuncaddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfuncaddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfuncaddr1`]
module"]
pub type TXFUNCADDR1 = crate::Reg<txfuncaddr1::TXFUNCADDR1_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 1"]
pub mod txfuncaddr1;
#[doc = "TXHUBADDR1 (rw) register accessor: USB Transmit Hub Address Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubaddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubaddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubaddr1`]
module"]
pub type TXHUBADDR1 = crate::Reg<txhubaddr1::TXHUBADDR1_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 1"]
pub mod txhubaddr1;
#[doc = "TXHUBPORT1 (rw) register accessor: USB Transmit Hub Port Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubport1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubport1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubport1`]
module"]
pub type TXHUBPORT1 = crate::Reg<txhubport1::TXHUBPORT1_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 1"]
pub mod txhubport1;
#[doc = "RXFUNCADDR1 (rw) register accessor: USB Receive Functional Address Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfuncaddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfuncaddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfuncaddr1`]
module"]
pub type RXFUNCADDR1 = crate::Reg<rxfuncaddr1::RXFUNCADDR1_SPEC>;
#[doc = "USB Receive Functional Address Endpoint 1"]
pub mod rxfuncaddr1;
#[doc = "RXHUBADDR1 (rw) register accessor: USB Receive Hub Address Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubaddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubaddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubaddr1`]
module"]
pub type RXHUBADDR1 = crate::Reg<rxhubaddr1::RXHUBADDR1_SPEC>;
#[doc = "USB Receive Hub Address Endpoint 1"]
pub mod rxhubaddr1;
#[doc = "RXHUBPORT1 (rw) register accessor: USB Receive Hub Port Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubport1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubport1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubport1`]
module"]
pub type RXHUBPORT1 = crate::Reg<rxhubport1::RXHUBPORT1_SPEC>;
#[doc = "USB Receive Hub Port Endpoint 1"]
pub mod rxhubport1;
#[doc = "TXFUNCADDR2 (rw) register accessor: USB Transmit Functional Address Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfuncaddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfuncaddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfuncaddr2`]
module"]
pub type TXFUNCADDR2 = crate::Reg<txfuncaddr2::TXFUNCADDR2_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 2"]
pub mod txfuncaddr2;
#[doc = "TXHUBADDR2 (rw) register accessor: USB Transmit Hub Address Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubaddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubaddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubaddr2`]
module"]
pub type TXHUBADDR2 = crate::Reg<txhubaddr2::TXHUBADDR2_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 2"]
pub mod txhubaddr2;
#[doc = "TXHUBPORT2 (rw) register accessor: USB Transmit Hub Port Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubport2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubport2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubport2`]
module"]
pub type TXHUBPORT2 = crate::Reg<txhubport2::TXHUBPORT2_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 2"]
pub mod txhubport2;
#[doc = "RXFUNCADDR2 (rw) register accessor: USB Receive Functional Address Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfuncaddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfuncaddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfuncaddr2`]
module"]
pub type RXFUNCADDR2 = crate::Reg<rxfuncaddr2::RXFUNCADDR2_SPEC>;
#[doc = "USB Receive Functional Address Endpoint 2"]
pub mod rxfuncaddr2;
#[doc = "RXHUBADDR2 (rw) register accessor: USB Receive Hub Address Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubaddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubaddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubaddr2`]
module"]
pub type RXHUBADDR2 = crate::Reg<rxhubaddr2::RXHUBADDR2_SPEC>;
#[doc = "USB Receive Hub Address Endpoint 2"]
pub mod rxhubaddr2;
#[doc = "RXHUBPORT2 (rw) register accessor: USB Receive Hub Port Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubport2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubport2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubport2`]
module"]
pub type RXHUBPORT2 = crate::Reg<rxhubport2::RXHUBPORT2_SPEC>;
#[doc = "USB Receive Hub Port Endpoint 2"]
pub mod rxhubport2;
#[doc = "TXFUNCADDR3 (rw) register accessor: USB Transmit Functional Address Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfuncaddr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfuncaddr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfuncaddr3`]
module"]
pub type TXFUNCADDR3 = crate::Reg<txfuncaddr3::TXFUNCADDR3_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 3"]
pub mod txfuncaddr3;
#[doc = "TXHUBADDR3 (rw) register accessor: USB Transmit Hub Address Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubaddr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubaddr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubaddr3`]
module"]
pub type TXHUBADDR3 = crate::Reg<txhubaddr3::TXHUBADDR3_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 3"]
pub mod txhubaddr3;
#[doc = "TXHUBPORT3 (rw) register accessor: USB Transmit Hub Port Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubport3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubport3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubport3`]
module"]
pub type TXHUBPORT3 = crate::Reg<txhubport3::TXHUBPORT3_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 3"]
pub mod txhubport3;
#[doc = "RXFUNCADDR3 (rw) register accessor: USB Receive Functional Address Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfuncaddr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfuncaddr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfuncaddr3`]
module"]
pub type RXFUNCADDR3 = crate::Reg<rxfuncaddr3::RXFUNCADDR3_SPEC>;
#[doc = "USB Receive Functional Address Endpoint 3"]
pub mod rxfuncaddr3;
#[doc = "RXHUBADDR3 (rw) register accessor: USB Receive Hub Address Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubaddr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubaddr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubaddr3`]
module"]
pub type RXHUBADDR3 = crate::Reg<rxhubaddr3::RXHUBADDR3_SPEC>;
#[doc = "USB Receive Hub Address Endpoint 3"]
pub mod rxhubaddr3;
#[doc = "RXHUBPORT3 (rw) register accessor: USB Receive Hub Port Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubport3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubport3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubport3`]
module"]
pub type RXHUBPORT3 = crate::Reg<rxhubport3::RXHUBPORT3_SPEC>;
#[doc = "USB Receive Hub Port Endpoint 3"]
pub mod rxhubport3;
#[doc = "TXFUNCADDR4 (rw) register accessor: USB Transmit Functional Address Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfuncaddr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfuncaddr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfuncaddr4`]
module"]
pub type TXFUNCADDR4 = crate::Reg<txfuncaddr4::TXFUNCADDR4_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 4"]
pub mod txfuncaddr4;
#[doc = "TXHUBADDR4 (rw) register accessor: USB Transmit Hub Address Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubaddr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubaddr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubaddr4`]
module"]
pub type TXHUBADDR4 = crate::Reg<txhubaddr4::TXHUBADDR4_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 4"]
pub mod txhubaddr4;
#[doc = "TXHUBPORT4 (rw) register accessor: USB Transmit Hub Port Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubport4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubport4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubport4`]
module"]
pub type TXHUBPORT4 = crate::Reg<txhubport4::TXHUBPORT4_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 4"]
pub mod txhubport4;
#[doc = "RXFUNCADDR4 (rw) register accessor: USB Receive Functional Address Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfuncaddr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfuncaddr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfuncaddr4`]
module"]
pub type RXFUNCADDR4 = crate::Reg<rxfuncaddr4::RXFUNCADDR4_SPEC>;
#[doc = "USB Receive Functional Address Endpoint 4"]
pub mod rxfuncaddr4;
#[doc = "RXHUBADDR4 (rw) register accessor: USB Receive Hub Address Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubaddr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubaddr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubaddr4`]
module"]
pub type RXHUBADDR4 = crate::Reg<rxhubaddr4::RXHUBADDR4_SPEC>;
#[doc = "USB Receive Hub Address Endpoint 4"]
pub mod rxhubaddr4;
#[doc = "RXHUBPORT4 (rw) register accessor: USB Receive Hub Port Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubport4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubport4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubport4`]
module"]
pub type RXHUBPORT4 = crate::Reg<rxhubport4::RXHUBPORT4_SPEC>;
#[doc = "USB Receive Hub Port Endpoint 4"]
pub mod rxhubport4;
#[doc = "TXFUNCADDR5 (rw) register accessor: USB Transmit Functional Address Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfuncaddr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfuncaddr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfuncaddr5`]
module"]
pub type TXFUNCADDR5 = crate::Reg<txfuncaddr5::TXFUNCADDR5_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 5"]
pub mod txfuncaddr5;
#[doc = "TXHUBADDR5 (rw) register accessor: USB Transmit Hub Address Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubaddr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubaddr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubaddr5`]
module"]
pub type TXHUBADDR5 = crate::Reg<txhubaddr5::TXHUBADDR5_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 5"]
pub mod txhubaddr5;
#[doc = "TXHUBPORT5 (rw) register accessor: USB Transmit Hub Port Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubport5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubport5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubport5`]
module"]
pub type TXHUBPORT5 = crate::Reg<txhubport5::TXHUBPORT5_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 5"]
pub mod txhubport5;
#[doc = "RXFUNCADDR5 (rw) register accessor: USB Receive Functional Address Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfuncaddr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfuncaddr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfuncaddr5`]
module"]
pub type RXFUNCADDR5 = crate::Reg<rxfuncaddr5::RXFUNCADDR5_SPEC>;
#[doc = "USB Receive Functional Address Endpoint 5"]
pub mod rxfuncaddr5;
#[doc = "RXHUBADDR5 (rw) register accessor: USB Receive Hub Address Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubaddr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubaddr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubaddr5`]
module"]
pub type RXHUBADDR5 = crate::Reg<rxhubaddr5::RXHUBADDR5_SPEC>;
#[doc = "USB Receive Hub Address Endpoint 5"]
pub mod rxhubaddr5;
#[doc = "RXHUBPORT5 (rw) register accessor: USB Receive Hub Port Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubport5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubport5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubport5`]
module"]
pub type RXHUBPORT5 = crate::Reg<rxhubport5::RXHUBPORT5_SPEC>;
#[doc = "USB Receive Hub Port Endpoint 5"]
pub mod rxhubport5;
#[doc = "TXFUNCADDR6 (rw) register accessor: USB Transmit Functional Address Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfuncaddr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfuncaddr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfuncaddr6`]
module"]
pub type TXFUNCADDR6 = crate::Reg<txfuncaddr6::TXFUNCADDR6_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 6"]
pub mod txfuncaddr6;
#[doc = "TXHUBADDR6 (rw) register accessor: USB Transmit Hub Address Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubaddr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubaddr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubaddr6`]
module"]
pub type TXHUBADDR6 = crate::Reg<txhubaddr6::TXHUBADDR6_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 6"]
pub mod txhubaddr6;
#[doc = "TXHUBPORT6 (rw) register accessor: USB Transmit Hub Port Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubport6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubport6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubport6`]
module"]
pub type TXHUBPORT6 = crate::Reg<txhubport6::TXHUBPORT6_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 6"]
pub mod txhubport6;
#[doc = "RXFUNCADDR6 (rw) register accessor: USB Receive Functional Address Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfuncaddr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfuncaddr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfuncaddr6`]
module"]
pub type RXFUNCADDR6 = crate::Reg<rxfuncaddr6::RXFUNCADDR6_SPEC>;
#[doc = "USB Receive Functional Address Endpoint 6"]
pub mod rxfuncaddr6;
#[doc = "RXHUBADDR6 (rw) register accessor: USB Receive Hub Address Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubaddr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubaddr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubaddr6`]
module"]
pub type RXHUBADDR6 = crate::Reg<rxhubaddr6::RXHUBADDR6_SPEC>;
#[doc = "USB Receive Hub Address Endpoint 6"]
pub mod rxhubaddr6;
#[doc = "RXHUBPORT6 (rw) register accessor: USB Receive Hub Port Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubport6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubport6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubport6`]
module"]
pub type RXHUBPORT6 = crate::Reg<rxhubport6::RXHUBPORT6_SPEC>;
#[doc = "USB Receive Hub Port Endpoint 6"]
pub mod rxhubport6;
#[doc = "TXFUNCADDR7 (rw) register accessor: USB Transmit Functional Address Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfuncaddr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfuncaddr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfuncaddr7`]
module"]
pub type TXFUNCADDR7 = crate::Reg<txfuncaddr7::TXFUNCADDR7_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 7"]
pub mod txfuncaddr7;
#[doc = "TXHUBADDR7 (rw) register accessor: USB Transmit Hub Address Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubaddr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubaddr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubaddr7`]
module"]
pub type TXHUBADDR7 = crate::Reg<txhubaddr7::TXHUBADDR7_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 7"]
pub mod txhubaddr7;
#[doc = "TXHUBPORT7 (rw) register accessor: USB Transmit Hub Port Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhubport7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhubport7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhubport7`]
module"]
pub type TXHUBPORT7 = crate::Reg<txhubport7::TXHUBPORT7_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 7"]
pub mod txhubport7;
#[doc = "RXFUNCADDR7 (rw) register accessor: USB Receive Functional Address Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfuncaddr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfuncaddr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfuncaddr7`]
module"]
pub type RXFUNCADDR7 = crate::Reg<rxfuncaddr7::RXFUNCADDR7_SPEC>;
#[doc = "USB Receive Functional Address Endpoint 7"]
pub mod rxfuncaddr7;
#[doc = "RXHUBADDR7 (rw) register accessor: USB Receive Hub Address Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubaddr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubaddr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubaddr7`]
module"]
pub type RXHUBADDR7 = crate::Reg<rxhubaddr7::RXHUBADDR7_SPEC>;
#[doc = "USB Receive Hub Address Endpoint 7"]
pub mod rxhubaddr7;
#[doc = "RXHUBPORT7 (rw) register accessor: USB Receive Hub Port Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxhubport7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxhubport7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxhubport7`]
module"]
pub type RXHUBPORT7 = crate::Reg<rxhubport7::RXHUBPORT7_SPEC>;
#[doc = "USB Receive Hub Port Endpoint 7"]
pub mod rxhubport7;
#[doc = "CSRL0 (w) register accessor: USB Control and Status Endpoint 0 Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csrl0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csrl0`]
module"]
pub type CSRL0 = crate::Reg<csrl0::CSRL0_SPEC>;
#[doc = "USB Control and Status Endpoint 0 Low"]
pub mod csrl0;
#[doc = "USB0_ALT_CSRL0 (w) register accessor: USB Control and Status Endpoint 0 Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_csrl0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_csrl0`]
module"]
pub type USB0_ALT_CSRL0 = crate::Reg<usb0_alt_csrl0::USB0_ALT_CSRL0_SPEC>;
#[doc = "USB Control and Status Endpoint 0 Low"]
pub mod usb0_alt_csrl0;
#[doc = "CSRH0 (w) register accessor: USB Control and Status Endpoint 0 High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csrh0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csrh0`]
module"]
pub type CSRH0 = crate::Reg<csrh0::CSRH0_SPEC>;
#[doc = "USB Control and Status Endpoint 0 High"]
pub mod csrh0;
#[doc = "COUNT0 (rw) register accessor: USB Receive Byte Count Endpoint 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`count0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count0`]
module"]
pub type COUNT0 = crate::Reg<count0::COUNT0_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 0"]
pub mod count0;
#[doc = "TYPE0 (rw) register accessor: USB Type Endpoint 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`type0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`type0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@type0`]
module"]
pub type TYPE0 = crate::Reg<type0::TYPE0_SPEC>;
#[doc = "USB Type Endpoint 0"]
pub mod type0;
#[doc = "NAKLMT (rw) register accessor: USB NAK Limit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`naklmt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`naklmt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@naklmt`]
module"]
pub type NAKLMT = crate::Reg<naklmt::NAKLMT_SPEC>;
#[doc = "USB NAK Limit"]
pub mod naklmt;
#[doc = "TXMAXP1 (rw) register accessor: USB Maximum Transmit Data Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmaxp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmaxp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmaxp1`]
module"]
pub type TXMAXP1 = crate::Reg<txmaxp1::TXMAXP1_SPEC>;
#[doc = "USB Maximum Transmit Data Endpoint 1"]
pub mod txmaxp1;
#[doc = "TXCSRL1 (rw) register accessor: USB Transmit Control and Status Endpoint 1 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcsrl1`]
module"]
pub type TXCSRL1 = crate::Reg<txcsrl1::TXCSRL1_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 1 Low"]
pub mod txcsrl1;
#[doc = "USB0_ALT_TXCSRL1 (rw) register accessor: USB Transmit Control and Status Endpoint 1 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_txcsrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_txcsrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_txcsrl1`]
module"]
pub type USB0_ALT_TXCSRL1 = crate::Reg<usb0_alt_txcsrl1::USB0_ALT_TXCSRL1_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 1 Low"]
pub mod usb0_alt_txcsrl1;
#[doc = "TXCSRH1 (rw) register accessor: USB Transmit Control and Status Endpoint 1 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrh1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrh1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcsrh1`]
module"]
pub type TXCSRH1 = crate::Reg<txcsrh1::TXCSRH1_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 1 High"]
pub mod txcsrh1;
#[doc = "RXMAXP1 (rw) register accessor: USB Maximum Receive Data Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmaxp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmaxp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmaxp1`]
module"]
pub type RXMAXP1 = crate::Reg<rxmaxp1::RXMAXP1_SPEC>;
#[doc = "USB Maximum Receive Data Endpoint 1"]
pub mod rxmaxp1;
#[doc = "RXCSRL1 (rw) register accessor: USB Receive Control and Status Endpoint 1 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcsrl1`]
module"]
pub type RXCSRL1 = crate::Reg<rxcsrl1::RXCSRL1_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 1 Low"]
pub mod rxcsrl1;
#[doc = "USB0_ALT_RXCSRL1 (rw) register accessor: USB Receive Control and Status Endpoint 1 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxcsrl1`]
module"]
pub type USB0_ALT_RXCSRL1 = crate::Reg<usb0_alt_rxcsrl1::USB0_ALT_RXCSRL1_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 1 Low"]
pub mod usb0_alt_rxcsrl1;
#[doc = "RXCSRH1 (rw) register accessor: USB Receive Control and Status Endpoint 1 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrh1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrh1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcsrh1`]
module"]
pub type RXCSRH1 = crate::Reg<rxcsrh1::RXCSRH1_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 1 High"]
pub mod rxcsrh1;
#[doc = "USB0_ALT_RXCSRH1 (rw) register accessor: USB Receive Control and Status Endpoint 1 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrh1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrh1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxcsrh1`]
module"]
pub type USB0_ALT_RXCSRH1 = crate::Reg<usb0_alt_rxcsrh1::USB0_ALT_RXCSRH1_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 1 High"]
pub mod usb0_alt_rxcsrh1;
#[doc = "RXCOUNT1 (rw) register accessor: USB Receive Byte Count Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcount1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcount1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcount1`]
module"]
pub type RXCOUNT1 = crate::Reg<rxcount1::RXCOUNT1_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 1"]
pub mod rxcount1;
#[doc = "TXTYPE1 (rw) register accessor: USB Host Transmit Configure Type Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtype1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtype1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtype1`]
module"]
pub type TXTYPE1 = crate::Reg<txtype1::TXTYPE1_SPEC>;
#[doc = "USB Host Transmit Configure Type Endpoint 1"]
pub mod txtype1;
#[doc = "TXINTERVAL1 (rw) register accessor: USB Host Transmit Interval Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txinterval1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txinterval1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txinterval1`]
module"]
pub type TXINTERVAL1 = crate::Reg<txinterval1::TXINTERVAL1_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 1"]
pub mod txinterval1;
#[doc = "USB0_ALT_TXINTERVAL1 (rw) register accessor: USB Host Transmit Interval Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_txinterval1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_txinterval1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_txinterval1`]
module"]
pub type USB0_ALT_TXINTERVAL1 = crate::Reg<usb0_alt_txinterval1::USB0_ALT_TXINTERVAL1_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 1"]
pub mod usb0_alt_txinterval1;
#[doc = "RXTYPE1 (rw) register accessor: USB Host Configure Receive Type Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtype1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxtype1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtype1`]
module"]
pub type RXTYPE1 = crate::Reg<rxtype1::RXTYPE1_SPEC>;
#[doc = "USB Host Configure Receive Type Endpoint 1"]
pub mod rxtype1;
#[doc = "RXINTERVAL1 (rw) register accessor: USB Host Receive Polling Interval Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxinterval1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxinterval1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxinterval1`]
module"]
pub type RXINTERVAL1 = crate::Reg<rxinterval1::RXINTERVAL1_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 1"]
pub mod rxinterval1;
#[doc = "USB0_ALT_RXINTERVAL1 (rw) register accessor: USB Host Receive Polling Interval Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxinterval1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxinterval1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxinterval1`]
module"]
pub type USB0_ALT_RXINTERVAL1 = crate::Reg<usb0_alt_rxinterval1::USB0_ALT_RXINTERVAL1_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 1"]
pub mod usb0_alt_rxinterval1;
#[doc = "TXMAXP2 (rw) register accessor: USB Maximum Transmit Data Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmaxp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmaxp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmaxp2`]
module"]
pub type TXMAXP2 = crate::Reg<txmaxp2::TXMAXP2_SPEC>;
#[doc = "USB Maximum Transmit Data Endpoint 2"]
pub mod txmaxp2;
#[doc = "TXCSRL2 (rw) register accessor: USB Transmit Control and Status Endpoint 2 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcsrl2`]
module"]
pub type TXCSRL2 = crate::Reg<txcsrl2::TXCSRL2_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 2 Low"]
pub mod txcsrl2;
#[doc = "USB0_ALT_TXCSRL2 (rw) register accessor: USB Transmit Control and Status Endpoint 2 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_txcsrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_txcsrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_txcsrl2`]
module"]
pub type USB0_ALT_TXCSRL2 = crate::Reg<usb0_alt_txcsrl2::USB0_ALT_TXCSRL2_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 2 Low"]
pub mod usb0_alt_txcsrl2;
#[doc = "TXCSRH2 (rw) register accessor: USB Transmit Control and Status Endpoint 2 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrh2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrh2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcsrh2`]
module"]
pub type TXCSRH2 = crate::Reg<txcsrh2::TXCSRH2_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 2 High"]
pub mod txcsrh2;
#[doc = "RXMAXP2 (rw) register accessor: USB Maximum Receive Data Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmaxp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmaxp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmaxp2`]
module"]
pub type RXMAXP2 = crate::Reg<rxmaxp2::RXMAXP2_SPEC>;
#[doc = "USB Maximum Receive Data Endpoint 2"]
pub mod rxmaxp2;
#[doc = "RXCSRL2 (rw) register accessor: USB Receive Control and Status Endpoint 2 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcsrl2`]
module"]
pub type RXCSRL2 = crate::Reg<rxcsrl2::RXCSRL2_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 2 Low"]
pub mod rxcsrl2;
#[doc = "USB0_ALT_RXCSRL2 (rw) register accessor: USB Receive Control and Status Endpoint 2 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxcsrl2`]
module"]
pub type USB0_ALT_RXCSRL2 = crate::Reg<usb0_alt_rxcsrl2::USB0_ALT_RXCSRL2_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 2 Low"]
pub mod usb0_alt_rxcsrl2;
#[doc = "RXCSRH2 (rw) register accessor: USB Receive Control and Status Endpoint 2 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrh2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrh2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcsrh2`]
module"]
pub type RXCSRH2 = crate::Reg<rxcsrh2::RXCSRH2_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 2 High"]
pub mod rxcsrh2;
#[doc = "USB0_ALT_RXCSRH2 (rw) register accessor: USB Receive Control and Status Endpoint 2 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrh2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrh2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxcsrh2`]
module"]
pub type USB0_ALT_RXCSRH2 = crate::Reg<usb0_alt_rxcsrh2::USB0_ALT_RXCSRH2_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 2 High"]
pub mod usb0_alt_rxcsrh2;
#[doc = "RXCOUNT2 (rw) register accessor: USB Receive Byte Count Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcount2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcount2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcount2`]
module"]
pub type RXCOUNT2 = crate::Reg<rxcount2::RXCOUNT2_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 2"]
pub mod rxcount2;
#[doc = "TXTYPE2 (rw) register accessor: USB Host Transmit Configure Type Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtype2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtype2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtype2`]
module"]
pub type TXTYPE2 = crate::Reg<txtype2::TXTYPE2_SPEC>;
#[doc = "USB Host Transmit Configure Type Endpoint 2"]
pub mod txtype2;
#[doc = "TXINTERVAL2 (rw) register accessor: USB Host Transmit Interval Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txinterval2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txinterval2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txinterval2`]
module"]
pub type TXINTERVAL2 = crate::Reg<txinterval2::TXINTERVAL2_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 2"]
pub mod txinterval2;
#[doc = "USB0_ALT_TXINTERVAL2 (rw) register accessor: USB Host Transmit Interval Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_txinterval2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_txinterval2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_txinterval2`]
module"]
pub type USB0_ALT_TXINTERVAL2 = crate::Reg<usb0_alt_txinterval2::USB0_ALT_TXINTERVAL2_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 2"]
pub mod usb0_alt_txinterval2;
#[doc = "RXTYPE2 (rw) register accessor: USB Host Configure Receive Type Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtype2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxtype2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtype2`]
module"]
pub type RXTYPE2 = crate::Reg<rxtype2::RXTYPE2_SPEC>;
#[doc = "USB Host Configure Receive Type Endpoint 2"]
pub mod rxtype2;
#[doc = "RXINTERVAL2 (rw) register accessor: USB Host Receive Polling Interval Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxinterval2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxinterval2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxinterval2`]
module"]
pub type RXINTERVAL2 = crate::Reg<rxinterval2::RXINTERVAL2_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 2"]
pub mod rxinterval2;
#[doc = "USB0_ALT_RXINTERVAL2 (rw) register accessor: USB Host Receive Polling Interval Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxinterval2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxinterval2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxinterval2`]
module"]
pub type USB0_ALT_RXINTERVAL2 = crate::Reg<usb0_alt_rxinterval2::USB0_ALT_RXINTERVAL2_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 2"]
pub mod usb0_alt_rxinterval2;
#[doc = "TXMAXP3 (rw) register accessor: USB Maximum Transmit Data Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmaxp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmaxp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmaxp3`]
module"]
pub type TXMAXP3 = crate::Reg<txmaxp3::TXMAXP3_SPEC>;
#[doc = "USB Maximum Transmit Data Endpoint 3"]
pub mod txmaxp3;
#[doc = "TXCSRL3 (rw) register accessor: USB Transmit Control and Status Endpoint 3 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcsrl3`]
module"]
pub type TXCSRL3 = crate::Reg<txcsrl3::TXCSRL3_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 3 Low"]
pub mod txcsrl3;
#[doc = "USB0_ALT_TXCSRL3 (rw) register accessor: USB Transmit Control and Status Endpoint 3 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_txcsrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_txcsrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_txcsrl3`]
module"]
pub type USB0_ALT_TXCSRL3 = crate::Reg<usb0_alt_txcsrl3::USB0_ALT_TXCSRL3_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 3 Low"]
pub mod usb0_alt_txcsrl3;
#[doc = "TXCSRH3 (rw) register accessor: USB Transmit Control and Status Endpoint 3 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrh3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrh3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcsrh3`]
module"]
pub type TXCSRH3 = crate::Reg<txcsrh3::TXCSRH3_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 3 High"]
pub mod txcsrh3;
#[doc = "RXMAXP3 (rw) register accessor: USB Maximum Receive Data Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmaxp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmaxp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmaxp3`]
module"]
pub type RXMAXP3 = crate::Reg<rxmaxp3::RXMAXP3_SPEC>;
#[doc = "USB Maximum Receive Data Endpoint 3"]
pub mod rxmaxp3;
#[doc = "RXCSRL3 (rw) register accessor: USB Receive Control and Status Endpoint 3 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcsrl3`]
module"]
pub type RXCSRL3 = crate::Reg<rxcsrl3::RXCSRL3_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 3 Low"]
pub mod rxcsrl3;
#[doc = "USB0_ALT_RXCSRL3 (rw) register accessor: USB Receive Control and Status Endpoint 3 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxcsrl3`]
module"]
pub type USB0_ALT_RXCSRL3 = crate::Reg<usb0_alt_rxcsrl3::USB0_ALT_RXCSRL3_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 3 Low"]
pub mod usb0_alt_rxcsrl3;
#[doc = "RXCSRH3 (rw) register accessor: USB Receive Control and Status Endpoint 3 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrh3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrh3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcsrh3`]
module"]
pub type RXCSRH3 = crate::Reg<rxcsrh3::RXCSRH3_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 3 High"]
pub mod rxcsrh3;
#[doc = "USB0_ALT_RXCSRH3 (rw) register accessor: USB Receive Control and Status Endpoint 3 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrh3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrh3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxcsrh3`]
module"]
pub type USB0_ALT_RXCSRH3 = crate::Reg<usb0_alt_rxcsrh3::USB0_ALT_RXCSRH3_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 3 High"]
pub mod usb0_alt_rxcsrh3;
#[doc = "RXCOUNT3 (rw) register accessor: USB Receive Byte Count Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcount3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcount3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcount3`]
module"]
pub type RXCOUNT3 = crate::Reg<rxcount3::RXCOUNT3_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 3"]
pub mod rxcount3;
#[doc = "TXTYPE3 (rw) register accessor: USB Host Transmit Configure Type Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtype3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtype3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtype3`]
module"]
pub type TXTYPE3 = crate::Reg<txtype3::TXTYPE3_SPEC>;
#[doc = "USB Host Transmit Configure Type Endpoint 3"]
pub mod txtype3;
#[doc = "TXINTERVAL3 (rw) register accessor: USB Host Transmit Interval Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txinterval3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txinterval3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txinterval3`]
module"]
pub type TXINTERVAL3 = crate::Reg<txinterval3::TXINTERVAL3_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 3"]
pub mod txinterval3;
#[doc = "USB0_ALT_TXINTERVAL3 (rw) register accessor: USB Host Transmit Interval Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_txinterval3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_txinterval3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_txinterval3`]
module"]
pub type USB0_ALT_TXINTERVAL3 = crate::Reg<usb0_alt_txinterval3::USB0_ALT_TXINTERVAL3_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 3"]
pub mod usb0_alt_txinterval3;
#[doc = "RXTYPE3 (rw) register accessor: USB Host Configure Receive Type Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtype3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxtype3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtype3`]
module"]
pub type RXTYPE3 = crate::Reg<rxtype3::RXTYPE3_SPEC>;
#[doc = "USB Host Configure Receive Type Endpoint 3"]
pub mod rxtype3;
#[doc = "RXINTERVAL3 (rw) register accessor: USB Host Receive Polling Interval Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxinterval3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxinterval3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxinterval3`]
module"]
pub type RXINTERVAL3 = crate::Reg<rxinterval3::RXINTERVAL3_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 3"]
pub mod rxinterval3;
#[doc = "USB0_ALT_RXINTERVAL3 (rw) register accessor: USB Host Receive Polling Interval Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxinterval3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxinterval3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxinterval3`]
module"]
pub type USB0_ALT_RXINTERVAL3 = crate::Reg<usb0_alt_rxinterval3::USB0_ALT_RXINTERVAL3_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 3"]
pub mod usb0_alt_rxinterval3;
#[doc = "TXMAXP4 (rw) register accessor: USB Maximum Transmit Data Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmaxp4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmaxp4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmaxp4`]
module"]
pub type TXMAXP4 = crate::Reg<txmaxp4::TXMAXP4_SPEC>;
#[doc = "USB Maximum Transmit Data Endpoint 4"]
pub mod txmaxp4;
#[doc = "TXCSRL4 (rw) register accessor: USB Transmit Control and Status Endpoint 4 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcsrl4`]
module"]
pub type TXCSRL4 = crate::Reg<txcsrl4::TXCSRL4_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 4 Low"]
pub mod txcsrl4;
#[doc = "USB0_ALT_TXCSRL4 (rw) register accessor: USB Transmit Control and Status Endpoint 4 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_txcsrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_txcsrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_txcsrl4`]
module"]
pub type USB0_ALT_TXCSRL4 = crate::Reg<usb0_alt_txcsrl4::USB0_ALT_TXCSRL4_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 4 Low"]
pub mod usb0_alt_txcsrl4;
#[doc = "TXCSRH4 (rw) register accessor: USB Transmit Control and Status Endpoint 4 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrh4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrh4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcsrh4`]
module"]
pub type TXCSRH4 = crate::Reg<txcsrh4::TXCSRH4_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 4 High"]
pub mod txcsrh4;
#[doc = "RXMAXP4 (rw) register accessor: USB Maximum Receive Data Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmaxp4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmaxp4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmaxp4`]
module"]
pub type RXMAXP4 = crate::Reg<rxmaxp4::RXMAXP4_SPEC>;
#[doc = "USB Maximum Receive Data Endpoint 4"]
pub mod rxmaxp4;
#[doc = "RXCSRL4 (rw) register accessor: USB Receive Control and Status Endpoint 4 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcsrl4`]
module"]
pub type RXCSRL4 = crate::Reg<rxcsrl4::RXCSRL4_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 4 Low"]
pub mod rxcsrl4;
#[doc = "USB0_ALT_RXCSRL4 (rw) register accessor: USB Receive Control and Status Endpoint 4 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxcsrl4`]
module"]
pub type USB0_ALT_RXCSRL4 = crate::Reg<usb0_alt_rxcsrl4::USB0_ALT_RXCSRL4_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 4 Low"]
pub mod usb0_alt_rxcsrl4;
#[doc = "RXCSRH4 (rw) register accessor: USB Receive Control and Status Endpoint 4 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrh4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrh4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcsrh4`]
module"]
pub type RXCSRH4 = crate::Reg<rxcsrh4::RXCSRH4_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 4 High"]
pub mod rxcsrh4;
#[doc = "USB0_ALT_RXCSRH4 (rw) register accessor: USB Receive Control and Status Endpoint 4 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrh4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrh4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxcsrh4`]
module"]
pub type USB0_ALT_RXCSRH4 = crate::Reg<usb0_alt_rxcsrh4::USB0_ALT_RXCSRH4_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 4 High"]
pub mod usb0_alt_rxcsrh4;
#[doc = "RXCOUNT4 (rw) register accessor: USB Receive Byte Count Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcount4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcount4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcount4`]
module"]
pub type RXCOUNT4 = crate::Reg<rxcount4::RXCOUNT4_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 4"]
pub mod rxcount4;
#[doc = "TXTYPE4 (rw) register accessor: USB Host Transmit Configure Type Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtype4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtype4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtype4`]
module"]
pub type TXTYPE4 = crate::Reg<txtype4::TXTYPE4_SPEC>;
#[doc = "USB Host Transmit Configure Type Endpoint 4"]
pub mod txtype4;
#[doc = "TXINTERVAL4 (rw) register accessor: USB Host Transmit Interval Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txinterval4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txinterval4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txinterval4`]
module"]
pub type TXINTERVAL4 = crate::Reg<txinterval4::TXINTERVAL4_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 4"]
pub mod txinterval4;
#[doc = "USB0_ALT_TXINTERVAL4 (rw) register accessor: USB Host Transmit Interval Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_txinterval4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_txinterval4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_txinterval4`]
module"]
pub type USB0_ALT_TXINTERVAL4 = crate::Reg<usb0_alt_txinterval4::USB0_ALT_TXINTERVAL4_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 4"]
pub mod usb0_alt_txinterval4;
#[doc = "RXTYPE4 (rw) register accessor: USB Host Configure Receive Type Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtype4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxtype4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtype4`]
module"]
pub type RXTYPE4 = crate::Reg<rxtype4::RXTYPE4_SPEC>;
#[doc = "USB Host Configure Receive Type Endpoint 4"]
pub mod rxtype4;
#[doc = "RXINTERVAL4 (rw) register accessor: USB Host Receive Polling Interval Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxinterval4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxinterval4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxinterval4`]
module"]
pub type RXINTERVAL4 = crate::Reg<rxinterval4::RXINTERVAL4_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 4"]
pub mod rxinterval4;
#[doc = "USB0_ALT_RXINTERVAL4 (rw) register accessor: USB Host Receive Polling Interval Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxinterval4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxinterval4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxinterval4`]
module"]
pub type USB0_ALT_RXINTERVAL4 = crate::Reg<usb0_alt_rxinterval4::USB0_ALT_RXINTERVAL4_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 4"]
pub mod usb0_alt_rxinterval4;
#[doc = "TXMAXP5 (rw) register accessor: USB Maximum Transmit Data Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmaxp5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmaxp5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmaxp5`]
module"]
pub type TXMAXP5 = crate::Reg<txmaxp5::TXMAXP5_SPEC>;
#[doc = "USB Maximum Transmit Data Endpoint 5"]
pub mod txmaxp5;
#[doc = "TXCSRL5 (rw) register accessor: USB Transmit Control and Status Endpoint 5 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcsrl5`]
module"]
pub type TXCSRL5 = crate::Reg<txcsrl5::TXCSRL5_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 5 Low"]
pub mod txcsrl5;
#[doc = "USB0_ALT_TXCSRL5 (rw) register accessor: USB Transmit Control and Status Endpoint 5 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_txcsrl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_txcsrl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_txcsrl5`]
module"]
pub type USB0_ALT_TXCSRL5 = crate::Reg<usb0_alt_txcsrl5::USB0_ALT_TXCSRL5_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 5 Low"]
pub mod usb0_alt_txcsrl5;
#[doc = "TXCSRH5 (rw) register accessor: USB Transmit Control and Status Endpoint 5 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrh5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrh5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcsrh5`]
module"]
pub type TXCSRH5 = crate::Reg<txcsrh5::TXCSRH5_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 5 High"]
pub mod txcsrh5;
#[doc = "RXMAXP5 (rw) register accessor: USB Maximum Receive Data Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmaxp5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmaxp5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmaxp5`]
module"]
pub type RXMAXP5 = crate::Reg<rxmaxp5::RXMAXP5_SPEC>;
#[doc = "USB Maximum Receive Data Endpoint 5"]
pub mod rxmaxp5;
#[doc = "RXCSRL5 (rw) register accessor: USB Receive Control and Status Endpoint 5 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcsrl5`]
module"]
pub type RXCSRL5 = crate::Reg<rxcsrl5::RXCSRL5_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 5 Low"]
pub mod rxcsrl5;
#[doc = "USB0_ALT_RXCSRL5 (rw) register accessor: USB Receive Control and Status Endpoint 5 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxcsrl5`]
module"]
pub type USB0_ALT_RXCSRL5 = crate::Reg<usb0_alt_rxcsrl5::USB0_ALT_RXCSRL5_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 5 Low"]
pub mod usb0_alt_rxcsrl5;
#[doc = "RXCSRH5 (rw) register accessor: USB Receive Control and Status Endpoint 5 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrh5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrh5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcsrh5`]
module"]
pub type RXCSRH5 = crate::Reg<rxcsrh5::RXCSRH5_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 5 High"]
pub mod rxcsrh5;
#[doc = "USB0_ALT_RXCSRH5 (rw) register accessor: USB Receive Control and Status Endpoint 5 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrh5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrh5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxcsrh5`]
module"]
pub type USB0_ALT_RXCSRH5 = crate::Reg<usb0_alt_rxcsrh5::USB0_ALT_RXCSRH5_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 5 High"]
pub mod usb0_alt_rxcsrh5;
#[doc = "RXCOUNT5 (rw) register accessor: USB Receive Byte Count Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcount5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcount5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcount5`]
module"]
pub type RXCOUNT5 = crate::Reg<rxcount5::RXCOUNT5_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 5"]
pub mod rxcount5;
#[doc = "TXTYPE5 (rw) register accessor: USB Host Transmit Configure Type Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtype5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtype5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtype5`]
module"]
pub type TXTYPE5 = crate::Reg<txtype5::TXTYPE5_SPEC>;
#[doc = "USB Host Transmit Configure Type Endpoint 5"]
pub mod txtype5;
#[doc = "TXINTERVAL5 (rw) register accessor: USB Host Transmit Interval Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txinterval5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txinterval5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txinterval5`]
module"]
pub type TXINTERVAL5 = crate::Reg<txinterval5::TXINTERVAL5_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 5"]
pub mod txinterval5;
#[doc = "USB0_ALT_TXINTERVAL5 (rw) register accessor: USB Host Transmit Interval Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_txinterval5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_txinterval5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_txinterval5`]
module"]
pub type USB0_ALT_TXINTERVAL5 = crate::Reg<usb0_alt_txinterval5::USB0_ALT_TXINTERVAL5_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 5"]
pub mod usb0_alt_txinterval5;
#[doc = "RXTYPE5 (rw) register accessor: USB Host Configure Receive Type Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtype5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxtype5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtype5`]
module"]
pub type RXTYPE5 = crate::Reg<rxtype5::RXTYPE5_SPEC>;
#[doc = "USB Host Configure Receive Type Endpoint 5"]
pub mod rxtype5;
#[doc = "RXINTERVAL5 (rw) register accessor: USB Host Receive Polling Interval Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxinterval5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxinterval5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxinterval5`]
module"]
pub type RXINTERVAL5 = crate::Reg<rxinterval5::RXINTERVAL5_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 5"]
pub mod rxinterval5;
#[doc = "USB0_ALT_RXINTERVAL5 (rw) register accessor: USB Host Receive Polling Interval Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxinterval5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxinterval5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxinterval5`]
module"]
pub type USB0_ALT_RXINTERVAL5 = crate::Reg<usb0_alt_rxinterval5::USB0_ALT_RXINTERVAL5_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 5"]
pub mod usb0_alt_rxinterval5;
#[doc = "TXMAXP6 (rw) register accessor: USB Maximum Transmit Data Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmaxp6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmaxp6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmaxp6`]
module"]
pub type TXMAXP6 = crate::Reg<txmaxp6::TXMAXP6_SPEC>;
#[doc = "USB Maximum Transmit Data Endpoint 6"]
pub mod txmaxp6;
#[doc = "TXCSRL6 (rw) register accessor: USB Transmit Control and Status Endpoint 6 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcsrl6`]
module"]
pub type TXCSRL6 = crate::Reg<txcsrl6::TXCSRL6_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 6 Low"]
pub mod txcsrl6;
#[doc = "USB0_ALT_TXCSRL6 (rw) register accessor: USB Transmit Control and Status Endpoint 6 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_txcsrl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_txcsrl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_txcsrl6`]
module"]
pub type USB0_ALT_TXCSRL6 = crate::Reg<usb0_alt_txcsrl6::USB0_ALT_TXCSRL6_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 6 Low"]
pub mod usb0_alt_txcsrl6;
#[doc = "TXCSRH6 (rw) register accessor: USB Transmit Control and Status Endpoint 6 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrh6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrh6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcsrh6`]
module"]
pub type TXCSRH6 = crate::Reg<txcsrh6::TXCSRH6_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 6 High"]
pub mod txcsrh6;
#[doc = "RXMAXP6 (rw) register accessor: USB Maximum Receive Data Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmaxp6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmaxp6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmaxp6`]
module"]
pub type RXMAXP6 = crate::Reg<rxmaxp6::RXMAXP6_SPEC>;
#[doc = "USB Maximum Receive Data Endpoint 6"]
pub mod rxmaxp6;
#[doc = "RXCSRL6 (rw) register accessor: USB Receive Control and Status Endpoint 6 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcsrl6`]
module"]
pub type RXCSRL6 = crate::Reg<rxcsrl6::RXCSRL6_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 6 Low"]
pub mod rxcsrl6;
#[doc = "USB0_ALT_RXCSRL6 (rw) register accessor: USB Receive Control and Status Endpoint 6 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxcsrl6`]
module"]
pub type USB0_ALT_RXCSRL6 = crate::Reg<usb0_alt_rxcsrl6::USB0_ALT_RXCSRL6_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 6 Low"]
pub mod usb0_alt_rxcsrl6;
#[doc = "RXCSRH6 (rw) register accessor: USB Receive Control and Status Endpoint 6 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrh6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrh6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcsrh6`]
module"]
pub type RXCSRH6 = crate::Reg<rxcsrh6::RXCSRH6_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 6 High"]
pub mod rxcsrh6;
#[doc = "USB0_ALT_RXCSRH6 (rw) register accessor: USB Receive Control and Status Endpoint 6 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrh6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrh6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxcsrh6`]
module"]
pub type USB0_ALT_RXCSRH6 = crate::Reg<usb0_alt_rxcsrh6::USB0_ALT_RXCSRH6_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 6 High"]
pub mod usb0_alt_rxcsrh6;
#[doc = "RXCOUNT6 (rw) register accessor: USB Receive Byte Count Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcount6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcount6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcount6`]
module"]
pub type RXCOUNT6 = crate::Reg<rxcount6::RXCOUNT6_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 6"]
pub mod rxcount6;
#[doc = "TXTYPE6 (rw) register accessor: USB Host Transmit Configure Type Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtype6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtype6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtype6`]
module"]
pub type TXTYPE6 = crate::Reg<txtype6::TXTYPE6_SPEC>;
#[doc = "USB Host Transmit Configure Type Endpoint 6"]
pub mod txtype6;
#[doc = "TXINTERVAL6 (rw) register accessor: USB Host Transmit Interval Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txinterval6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txinterval6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txinterval6`]
module"]
pub type TXINTERVAL6 = crate::Reg<txinterval6::TXINTERVAL6_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 6"]
pub mod txinterval6;
#[doc = "USB0_ALT_TXINTERVAL6 (rw) register accessor: USB Host Transmit Interval Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_txinterval6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_txinterval6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_txinterval6`]
module"]
pub type USB0_ALT_TXINTERVAL6 = crate::Reg<usb0_alt_txinterval6::USB0_ALT_TXINTERVAL6_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 6"]
pub mod usb0_alt_txinterval6;
#[doc = "RXTYPE6 (rw) register accessor: USB Host Configure Receive Type Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtype6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxtype6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtype6`]
module"]
pub type RXTYPE6 = crate::Reg<rxtype6::RXTYPE6_SPEC>;
#[doc = "USB Host Configure Receive Type Endpoint 6"]
pub mod rxtype6;
#[doc = "RXINTERVAL6 (rw) register accessor: USB Host Receive Polling Interval Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxinterval6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxinterval6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxinterval6`]
module"]
pub type RXINTERVAL6 = crate::Reg<rxinterval6::RXINTERVAL6_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 6"]
pub mod rxinterval6;
#[doc = "USB0_ALT_RXINTERVAL6 (rw) register accessor: USB Host Receive Polling Interval Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxinterval6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxinterval6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxinterval6`]
module"]
pub type USB0_ALT_RXINTERVAL6 = crate::Reg<usb0_alt_rxinterval6::USB0_ALT_RXINTERVAL6_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 6"]
pub mod usb0_alt_rxinterval6;
#[doc = "TXMAXP7 (rw) register accessor: USB Maximum Transmit Data Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmaxp7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmaxp7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmaxp7`]
module"]
pub type TXMAXP7 = crate::Reg<txmaxp7::TXMAXP7_SPEC>;
#[doc = "USB Maximum Transmit Data Endpoint 7"]
pub mod txmaxp7;
#[doc = "TXCSRL7 (rw) register accessor: USB Transmit Control and Status Endpoint 7 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcsrl7`]
module"]
pub type TXCSRL7 = crate::Reg<txcsrl7::TXCSRL7_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 7 Low"]
pub mod txcsrl7;
#[doc = "USB0_ALT_TXCSRL7 (rw) register accessor: USB Transmit Control and Status Endpoint 7 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_txcsrl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_txcsrl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_txcsrl7`]
module"]
pub type USB0_ALT_TXCSRL7 = crate::Reg<usb0_alt_txcsrl7::USB0_ALT_TXCSRL7_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 7 Low"]
pub mod usb0_alt_txcsrl7;
#[doc = "TXCSRH7 (rw) register accessor: USB Transmit Control and Status Endpoint 7 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcsrh7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcsrh7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcsrh7`]
module"]
pub type TXCSRH7 = crate::Reg<txcsrh7::TXCSRH7_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 7 High"]
pub mod txcsrh7;
#[doc = "RXMAXP7 (rw) register accessor: USB Maximum Receive Data Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmaxp7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmaxp7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmaxp7`]
module"]
pub type RXMAXP7 = crate::Reg<rxmaxp7::RXMAXP7_SPEC>;
#[doc = "USB Maximum Receive Data Endpoint 7"]
pub mod rxmaxp7;
#[doc = "RXCSRL7 (rw) register accessor: USB Receive Control and Status Endpoint 7 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcsrl7`]
module"]
pub type RXCSRL7 = crate::Reg<rxcsrl7::RXCSRL7_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 7 Low"]
pub mod rxcsrl7;
#[doc = "USB0_ALT_RXCSRL7 (rw) register accessor: USB Receive Control and Status Endpoint 7 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxcsrl7`]
module"]
pub type USB0_ALT_RXCSRL7 = crate::Reg<usb0_alt_rxcsrl7::USB0_ALT_RXCSRL7_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 7 Low"]
pub mod usb0_alt_rxcsrl7;
#[doc = "RXCSRH7 (rw) register accessor: USB Receive Control and Status Endpoint 7 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcsrh7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcsrh7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcsrh7`]
module"]
pub type RXCSRH7 = crate::Reg<rxcsrh7::RXCSRH7_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 7 High"]
pub mod rxcsrh7;
#[doc = "USB0_ALT_RXCSRH7 (rw) register accessor: USB Receive Control and Status Endpoint 7 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxcsrh7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxcsrh7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxcsrh7`]
module"]
pub type USB0_ALT_RXCSRH7 = crate::Reg<usb0_alt_rxcsrh7::USB0_ALT_RXCSRH7_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 7 High"]
pub mod usb0_alt_rxcsrh7;
#[doc = "RXCOUNT7 (rw) register accessor: USB Receive Byte Count Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcount7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcount7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcount7`]
module"]
pub type RXCOUNT7 = crate::Reg<rxcount7::RXCOUNT7_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 7"]
pub mod rxcount7;
#[doc = "TXTYPE7 (rw) register accessor: USB Host Transmit Configure Type Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txtype7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txtype7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtype7`]
module"]
pub type TXTYPE7 = crate::Reg<txtype7::TXTYPE7_SPEC>;
#[doc = "USB Host Transmit Configure Type Endpoint 7"]
pub mod txtype7;
#[doc = "TXINTERVAL7 (rw) register accessor: USB Host Transmit Interval Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txinterval7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txinterval7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txinterval7`]
module"]
pub type TXINTERVAL7 = crate::Reg<txinterval7::TXINTERVAL7_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 7"]
pub mod txinterval7;
#[doc = "USB0_ALT_TXINTERVAL7 (rw) register accessor: USB Host Transmit Interval Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_txinterval7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_txinterval7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_txinterval7`]
module"]
pub type USB0_ALT_TXINTERVAL7 = crate::Reg<usb0_alt_txinterval7::USB0_ALT_TXINTERVAL7_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 7"]
pub mod usb0_alt_txinterval7;
#[doc = "RXTYPE7 (rw) register accessor: USB Host Configure Receive Type Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxtype7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxtype7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtype7`]
module"]
pub type RXTYPE7 = crate::Reg<rxtype7::RXTYPE7_SPEC>;
#[doc = "USB Host Configure Receive Type Endpoint 7"]
pub mod rxtype7;
#[doc = "RXINTERVAL7 (rw) register accessor: USB Host Receive Polling Interval Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxinterval7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxinterval7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxinterval7`]
module"]
pub type RXINTERVAL7 = crate::Reg<rxinterval7::RXINTERVAL7_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 7"]
pub mod rxinterval7;
#[doc = "USB0_ALT_RXINTERVAL7 (rw) register accessor: USB Host Receive Polling Interval Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb0_alt_rxinterval7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb0_alt_rxinterval7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0_alt_rxinterval7`]
module"]
pub type USB0_ALT_RXINTERVAL7 = crate::Reg<usb0_alt_rxinterval7::USB0_ALT_RXINTERVAL7_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 7"]
pub mod usb0_alt_rxinterval7;
#[doc = "RQPKTCOUNT1 (rw) register accessor: USB Request Packet Count in Block Transfer Endpoint 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rqpktcount1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqpktcount1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rqpktcount1`]
module"]
pub type RQPKTCOUNT1 = crate::Reg<rqpktcount1::RQPKTCOUNT1_SPEC>;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 1"]
pub mod rqpktcount1;
#[doc = "RQPKTCOUNT2 (rw) register accessor: USB Request Packet Count in Block Transfer Endpoint 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rqpktcount2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqpktcount2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rqpktcount2`]
module"]
pub type RQPKTCOUNT2 = crate::Reg<rqpktcount2::RQPKTCOUNT2_SPEC>;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 2"]
pub mod rqpktcount2;
#[doc = "RQPKTCOUNT3 (rw) register accessor: USB Request Packet Count in Block Transfer Endpoint 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rqpktcount3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqpktcount3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rqpktcount3`]
module"]
pub type RQPKTCOUNT3 = crate::Reg<rqpktcount3::RQPKTCOUNT3_SPEC>;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 3"]
pub mod rqpktcount3;
#[doc = "RQPKTCOUNT4 (rw) register accessor: USB Request Packet Count in Block Transfer Endpoint 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rqpktcount4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqpktcount4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rqpktcount4`]
module"]
pub type RQPKTCOUNT4 = crate::Reg<rqpktcount4::RQPKTCOUNT4_SPEC>;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 4"]
pub mod rqpktcount4;
#[doc = "RQPKTCOUNT5 (rw) register accessor: USB Request Packet Count in Block Transfer Endpoint 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rqpktcount5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqpktcount5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rqpktcount5`]
module"]
pub type RQPKTCOUNT5 = crate::Reg<rqpktcount5::RQPKTCOUNT5_SPEC>;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 5"]
pub mod rqpktcount5;
#[doc = "RQPKTCOUNT6 (rw) register accessor: USB Request Packet Count in Block Transfer Endpoint 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rqpktcount6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqpktcount6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rqpktcount6`]
module"]
pub type RQPKTCOUNT6 = crate::Reg<rqpktcount6::RQPKTCOUNT6_SPEC>;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 6"]
pub mod rqpktcount6;
#[doc = "RQPKTCOUNT7 (rw) register accessor: USB Request Packet Count in Block Transfer Endpoint 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rqpktcount7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqpktcount7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rqpktcount7`]
module"]
pub type RQPKTCOUNT7 = crate::Reg<rqpktcount7::RQPKTCOUNT7_SPEC>;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 7"]
pub mod rqpktcount7;
#[doc = "RXDPKTBUFDIS (rw) register accessor: USB Receive Double Packet Buffer Disable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdpktbufdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdpktbufdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdpktbufdis`]
module"]
pub type RXDPKTBUFDIS = crate::Reg<rxdpktbufdis::RXDPKTBUFDIS_SPEC>;
#[doc = "USB Receive Double Packet Buffer Disable"]
pub mod rxdpktbufdis;
#[doc = "TXDPKTBUFDIS (rw) register accessor: USB Transmit Double Packet Buffer Disable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdpktbufdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdpktbufdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdpktbufdis`]
module"]
pub type TXDPKTBUFDIS = crate::Reg<txdpktbufdis::TXDPKTBUFDIS_SPEC>;
#[doc = "USB Transmit Double Packet Buffer Disable"]
pub mod txdpktbufdis;
#[doc = "EPC (rw) register accessor: USB External Power Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epc`]
module"]
pub type EPC = crate::Reg<epc::EPC_SPEC>;
#[doc = "USB External Power Control"]
pub mod epc;
#[doc = "EPCRIS (rw) register accessor: USB External Power Control Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epcris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epcris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epcris`]
module"]
pub type EPCRIS = crate::Reg<epcris::EPCRIS_SPEC>;
#[doc = "USB External Power Control Raw Interrupt Status"]
pub mod epcris;
#[doc = "EPCIM (rw) register accessor: USB External Power Control Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epcim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epcim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epcim`]
module"]
pub type EPCIM = crate::Reg<epcim::EPCIM_SPEC>;
#[doc = "USB External Power Control Interrupt Mask"]
pub mod epcim;
#[doc = "EPCISC (rw) register accessor: USB External Power Control Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epcisc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epcisc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epcisc`]
module"]
pub type EPCISC = crate::Reg<epcisc::EPCISC_SPEC>;
#[doc = "USB External Power Control Interrupt Status and Clear"]
pub mod epcisc;
#[doc = "DRRIS (rw) register accessor: USB Device RESUME Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drris`]
module"]
pub type DRRIS = crate::Reg<drris::DRRIS_SPEC>;
#[doc = "USB Device RESUME Raw Interrupt Status"]
pub mod drris;
#[doc = "DRIM (rw) register accessor: USB Device RESUME Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drim`]
module"]
pub type DRIM = crate::Reg<drim::DRIM_SPEC>;
#[doc = "USB Device RESUME Interrupt Mask"]
pub mod drim;
#[doc = "DRISC (w) register accessor: USB Device RESUME Interrupt Status and Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drisc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drisc`]
module"]
pub type DRISC = crate::Reg<drisc::DRISC_SPEC>;
#[doc = "USB Device RESUME Interrupt Status and Clear"]
pub mod drisc;
#[doc = "GPCS (rw) register accessor: USB General-Purpose Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpcs`]
module"]
pub type GPCS = crate::Reg<gpcs::GPCS_SPEC>;
#[doc = "USB General-Purpose Control and Status"]
pub mod gpcs;
#[doc = "VDC (rw) register accessor: USB VBUS Droop Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdc`]
module"]
pub type VDC = crate::Reg<vdc::VDC_SPEC>;
#[doc = "USB VBUS Droop Control"]
pub mod vdc;
#[doc = "VDCRIS (rw) register accessor: USB VBUS Droop Control Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdcris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdcris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdcris`]
module"]
pub type VDCRIS = crate::Reg<vdcris::VDCRIS_SPEC>;
#[doc = "USB VBUS Droop Control Raw Interrupt Status"]
pub mod vdcris;
#[doc = "VDCIM (rw) register accessor: USB VBUS Droop Control Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdcim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdcim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdcim`]
module"]
pub type VDCIM = crate::Reg<vdcim::VDCIM_SPEC>;
#[doc = "USB VBUS Droop Control Interrupt Mask"]
pub mod vdcim;
#[doc = "VDCISC (rw) register accessor: USB VBUS Droop Control Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdcisc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdcisc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdcisc`]
module"]
pub type VDCISC = crate::Reg<vdcisc::VDCISC_SPEC>;
#[doc = "USB VBUS Droop Control Interrupt Status and Clear"]
pub mod vdcisc;
#[doc = "IDVRIS (rw) register accessor: USB ID Valid Detect Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idvris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idvris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idvris`]
module"]
pub type IDVRIS = crate::Reg<idvris::IDVRIS_SPEC>;
#[doc = "USB ID Valid Detect Raw Interrupt Status"]
pub mod idvris;
#[doc = "IDVIM (rw) register accessor: USB ID Valid Detect Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idvim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idvim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idvim`]
module"]
pub type IDVIM = crate::Reg<idvim::IDVIM_SPEC>;
#[doc = "USB ID Valid Detect Interrupt Mask"]
pub mod idvim;
#[doc = "IDVISC (rw) register accessor: USB ID Valid Detect Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idvisc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idvisc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idvisc`]
module"]
pub type IDVISC = crate::Reg<idvisc::IDVISC_SPEC>;
#[doc = "USB ID Valid Detect Interrupt Status and Clear"]
pub mod idvisc;
#[doc = "DMASEL (rw) register accessor: USB DMA Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasel`]
module"]
pub type DMASEL = crate::Reg<dmasel::DMASEL_SPEC>;
#[doc = "USB DMA Select"]
pub mod dmasel;
#[doc = "PP (rw) register accessor: USB Peripheral Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pp`]
module"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "USB Peripheral Properties"]
pub mod pp;
