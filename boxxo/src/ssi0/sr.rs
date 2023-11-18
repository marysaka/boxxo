#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `SSI_SR_TFE` reader - SSI Transmit FIFO Empty"]
pub type SSI_SR_TFE_R = crate::BitReader;
#[doc = "Field `SSI_SR_TFE` writer - SSI Transmit FIFO Empty"]
pub type SSI_SR_TFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_SR_TNF` reader - SSI Transmit FIFO Not Full"]
pub type SSI_SR_TNF_R = crate::BitReader;
#[doc = "Field `SSI_SR_TNF` writer - SSI Transmit FIFO Not Full"]
pub type SSI_SR_TNF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_SR_RNE` reader - SSI Receive FIFO Not Empty"]
pub type SSI_SR_RNE_R = crate::BitReader;
#[doc = "Field `SSI_SR_RNE` writer - SSI Receive FIFO Not Empty"]
pub type SSI_SR_RNE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_SR_RFF` reader - SSI Receive FIFO Full"]
pub type SSI_SR_RFF_R = crate::BitReader;
#[doc = "Field `SSI_SR_RFF` writer - SSI Receive FIFO Full"]
pub type SSI_SR_RFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI_SR_BSY` reader - SSI Busy Bit"]
pub type SSI_SR_BSY_R = crate::BitReader;
#[doc = "Field `SSI_SR_BSY` writer - SSI Busy Bit"]
pub type SSI_SR_BSY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SSI Transmit FIFO Empty"]
    #[inline(always)]
    pub fn ssi_sr_tfe(&self) -> SSI_SR_TFE_R {
        SSI_SR_TFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Transmit FIFO Not Full"]
    #[inline(always)]
    pub fn ssi_sr_tnf(&self) -> SSI_SR_TNF_R {
        SSI_SR_TNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Not Empty"]
    #[inline(always)]
    pub fn ssi_sr_rne(&self) -> SSI_SR_RNE_R {
        SSI_SR_RNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Receive FIFO Full"]
    #[inline(always)]
    pub fn ssi_sr_rff(&self) -> SSI_SR_RFF_R {
        SSI_SR_RFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SSI Busy Bit"]
    #[inline(always)]
    pub fn ssi_sr_bsy(&self) -> SSI_SR_BSY_R {
        SSI_SR_BSY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Transmit FIFO Empty"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_sr_tfe(&mut self) -> SSI_SR_TFE_W<SR_SPEC, 0> {
        SSI_SR_TFE_W::new(self)
    }
    #[doc = "Bit 1 - SSI Transmit FIFO Not Full"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_sr_tnf(&mut self) -> SSI_SR_TNF_W<SR_SPEC, 1> {
        SSI_SR_TNF_W::new(self)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Not Empty"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_sr_rne(&mut self) -> SSI_SR_RNE_W<SR_SPEC, 2> {
        SSI_SR_RNE_W::new(self)
    }
    #[doc = "Bit 3 - SSI Receive FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_sr_rff(&mut self) -> SSI_SR_RFF_W<SR_SPEC, 3> {
        SSI_SR_RFF_W::new(self)
    }
    #[doc = "Bit 4 - SSI Busy Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_sr_bsy(&mut self) -> SSI_SR_BSY_W<SR_SPEC, 4> {
        SSI_SR_BSY_W::new(self)
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
#[doc = "SSI Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
