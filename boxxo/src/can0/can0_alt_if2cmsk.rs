#[doc = "Register `IF2CMSK` reader"]
pub type R = crate::R<CAN0_ALT_IF2CMSK_SPEC>;
#[doc = "Register `IF2CMSK` writer"]
pub type W = crate::W<CAN0_ALT_IF2CMSK_SPEC>;
#[doc = "Field `CAN_IF2CMSK_TXRQST` reader - Access Transmission Request"]
pub type CAN_IF2CMSK_TXRQST_R = crate::BitReader;
#[doc = "Field `CAN_IF2CMSK_TXRQST` writer - Access Transmission Request"]
pub type CAN_IF2CMSK_TXRQST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Access Transmission Request"]
    #[inline(always)]
    pub fn can_if2cmsk_txrqst(&self) -> CAN_IF2CMSK_TXRQST_R {
        CAN_IF2CMSK_TXRQST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Access Transmission Request"]
    #[inline(always)]
    #[must_use]
    pub fn can_if2cmsk_txrqst(&mut self) -> CAN_IF2CMSK_TXRQST_W<CAN0_ALT_IF2CMSK_SPEC, 2> {
        CAN_IF2CMSK_TXRQST_W::new(self)
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
#[doc = "CAN IF2 Command Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can0_alt_if2cmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can0_alt_if2cmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN0_ALT_IF2CMSK_SPEC;
impl crate::RegisterSpec for CAN0_ALT_IF2CMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can0_alt_if2cmsk::R`](R) reader structure"]
impl crate::Readable for CAN0_ALT_IF2CMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can0_alt_if2cmsk::W`](W) writer structure"]
impl crate::Writable for CAN0_ALT_IF2CMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF2CMSK to value 0"]
impl crate::Resettable for CAN0_ALT_IF2CMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
