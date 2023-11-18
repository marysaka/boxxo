#[doc = "Register `RIS` reader"]
pub type R = crate::R<RIS_SPEC>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RIS_SPEC>;
#[doc = "Field `SSI_RIS_RORRIS` reader - SSI Receive Overrun Raw Interrupt Status"]
pub type SSI_RIS_RORRIS_R = crate::BitReader;
#[doc = "Field `SSI_RIS_RORRIS` writer - SSI Receive Overrun Raw Interrupt Status"]
pub type SSI_RIS_RORRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_RIS_RTRIS` reader - SSI Receive Time-Out Raw Interrupt Status"]
pub type SSI_RIS_RTRIS_R = crate::BitReader;
#[doc = "Field `SSI_RIS_RTRIS` writer - SSI Receive Time-Out Raw Interrupt Status"]
pub type SSI_RIS_RTRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_RIS_RXRIS` reader - SSI Receive FIFO Raw Interrupt Status"]
pub type SSI_RIS_RXRIS_R = crate::BitReader;
#[doc = "Field `SSI_RIS_RXRIS` writer - SSI Receive FIFO Raw Interrupt Status"]
pub type SSI_RIS_RXRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_RIS_TXRIS` reader - SSI Transmit FIFO Raw Interrupt Status"]
pub type SSI_RIS_TXRIS_R = crate::BitReader;
#[doc = "Field `SSI_RIS_TXRIS` writer - SSI Transmit FIFO Raw Interrupt Status"]
pub type SSI_RIS_TXRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SSI Receive Overrun Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rorris(&self) -> SSI_RIS_RORRIS_R {
        SSI_RIS_RORRIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rtris(&self) -> SSI_RIS_RTRIS_R {
        SSI_RIS_RTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rxris(&self) -> SSI_RIS_RXRIS_R {
        SSI_RIS_RXRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_txris(&self) -> SSI_RIS_TXRIS_R {
        SSI_RIS_TXRIS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Receive Overrun Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_ris_rorris(&mut self) -> SSI_RIS_RORRIS_W<RIS_SPEC, 0> {
        SSI_RIS_RORRIS_W::new(self)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_ris_rtris(&mut self) -> SSI_RIS_RTRIS_W<RIS_SPEC, 1> {
        SSI_RIS_RTRIS_W::new(self)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_ris_rxris(&mut self) -> SSI_RIS_RXRIS_W<RIS_SPEC, 2> {
        SSI_RIS_RXRIS_W::new(self)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_ris_txris(&mut self) -> SSI_RIS_TXRIS_W<RIS_SPEC, 3> {
        SSI_RIS_TXRIS_W::new(self)
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
#[doc = "SSI Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
