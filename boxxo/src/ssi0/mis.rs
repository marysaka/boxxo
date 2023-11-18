#[doc = "Register `MIS` reader"]
pub type R = crate::R<MIS_SPEC>;
#[doc = "Register `MIS` writer"]
pub type W = crate::W<MIS_SPEC>;
#[doc = "Field `SSI_MIS_RORMIS` reader - SSI Receive Overrun Masked Interrupt Status"]
pub type SSI_MIS_RORMIS_R = crate::BitReader;
#[doc = "Field `SSI_MIS_RORMIS` writer - SSI Receive Overrun Masked Interrupt Status"]
pub type SSI_MIS_RORMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_MIS_RTMIS` reader - SSI Receive Time-Out Masked Interrupt Status"]
pub type SSI_MIS_RTMIS_R = crate::BitReader;
#[doc = "Field `SSI_MIS_RTMIS` writer - SSI Receive Time-Out Masked Interrupt Status"]
pub type SSI_MIS_RTMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_MIS_RXMIS` reader - SSI Receive FIFO Masked Interrupt Status"]
pub type SSI_MIS_RXMIS_R = crate::BitReader;
#[doc = "Field `SSI_MIS_RXMIS` writer - SSI Receive FIFO Masked Interrupt Status"]
pub type SSI_MIS_RXMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_MIS_TXMIS` reader - SSI Transmit FIFO Masked Interrupt Status"]
pub type SSI_MIS_TXMIS_R = crate::BitReader;
#[doc = "Field `SSI_MIS_TXMIS` writer - SSI Transmit FIFO Masked Interrupt Status"]
pub type SSI_MIS_TXMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SSI Receive Overrun Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rormis(&self) -> SSI_MIS_RORMIS_R {
        SSI_MIS_RORMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rtmis(&self) -> SSI_MIS_RTMIS_R {
        SSI_MIS_RTMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rxmis(&self) -> SSI_MIS_RXMIS_R {
        SSI_MIS_RXMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_txmis(&self) -> SSI_MIS_TXMIS_R {
        SSI_MIS_TXMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Receive Overrun Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_mis_rormis(&mut self) -> SSI_MIS_RORMIS_W<MIS_SPEC, 0> {
        SSI_MIS_RORMIS_W::new(self)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_mis_rtmis(&mut self) -> SSI_MIS_RTMIS_W<MIS_SPEC, 1> {
        SSI_MIS_RTMIS_W::new(self)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_mis_rxmis(&mut self) -> SSI_MIS_RXMIS_W<MIS_SPEC, 2> {
        SSI_MIS_RXMIS_W::new(self)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_mis_txmis(&mut self) -> SSI_MIS_TXMIS_W<MIS_SPEC, 3> {
        SSI_MIS_TXMIS_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SSI Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mis::W`](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
