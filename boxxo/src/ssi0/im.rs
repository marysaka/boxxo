#[doc = "Register `IM` reader"]
pub type R = crate::R<IM_SPEC>;
#[doc = "Register `IM` writer"]
pub type W = crate::W<IM_SPEC>;
#[doc = "Field `SSI_IM_RORIM` reader - SSI Receive Overrun Interrupt Mask"]
pub type SSI_IM_RORIM_R = crate::BitReader;
#[doc = "Field `SSI_IM_RORIM` writer - SSI Receive Overrun Interrupt Mask"]
pub type SSI_IM_RORIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_IM_RTIM` reader - SSI Receive Time-Out Interrupt Mask"]
pub type SSI_IM_RTIM_R = crate::BitReader;
#[doc = "Field `SSI_IM_RTIM` writer - SSI Receive Time-Out Interrupt Mask"]
pub type SSI_IM_RTIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_IM_RXIM` reader - SSI Receive FIFO Interrupt Mask"]
pub type SSI_IM_RXIM_R = crate::BitReader;
#[doc = "Field `SSI_IM_RXIM` writer - SSI Receive FIFO Interrupt Mask"]
pub type SSI_IM_RXIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_IM_TXIM` reader - SSI Transmit FIFO Interrupt Mask"]
pub type SSI_IM_TXIM_R = crate::BitReader;
#[doc = "Field `SSI_IM_TXIM` writer - SSI Transmit FIFO Interrupt Mask"]
pub type SSI_IM_TXIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SSI Receive Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rorim(&self) -> SSI_IM_RORIM_R {
        SSI_IM_RORIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rtim(&self) -> SSI_IM_RTIM_R {
        SSI_IM_RTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rxim(&self) -> SSI_IM_RXIM_R {
        SSI_IM_RXIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_txim(&self) -> SSI_IM_TXIM_R {
        SSI_IM_TXIM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Receive Overrun Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_im_rorim(&mut self) -> SSI_IM_RORIM_W<IM_SPEC, 0> {
        SSI_IM_RORIM_W::new(self)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_im_rtim(&mut self) -> SSI_IM_RTIM_W<IM_SPEC, 1> {
        SSI_IM_RTIM_W::new(self)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_im_rxim(&mut self) -> SSI_IM_RXIM_W<IM_SPEC, 2> {
        SSI_IM_RXIM_W::new(self)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_im_txim(&mut self) -> SSI_IM_TXIM_W<IM_SPEC, 3> {
        SSI_IM_TXIM_W::new(self)
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
#[doc = "SSI Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`im::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`im::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`im::R`](R) reader structure"]
impl crate::Readable for IM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`im::W`](W) writer structure"]
impl crate::Writable for IM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
