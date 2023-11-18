#[doc = "Register `IDVRIS` reader"]
pub type R = crate::R<IDVRIS_SPEC>;
#[doc = "Register `IDVRIS` writer"]
pub type W = crate::W<IDVRIS_SPEC>;
#[doc = "Field `USB_IDVRIS_ID` reader - ID Valid Detect Raw Interrupt Status"]
pub type USB_IDVRIS_ID_R = crate::BitReader;
#[doc = "Field `USB_IDVRIS_ID` writer - ID Valid Detect Raw Interrupt Status"]
pub type USB_IDVRIS_ID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - ID Valid Detect Raw Interrupt Status"]
    #[inline(always)]
    pub fn usb_idvris_id(&self) -> USB_IDVRIS_ID_R {
        USB_IDVRIS_ID_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ID Valid Detect Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn usb_idvris_id(&mut self) -> USB_IDVRIS_ID_W<IDVRIS_SPEC, 0> {
        USB_IDVRIS_ID_W::new(self)
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
#[doc = "USB ID Valid Detect Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idvris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idvris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDVRIS_SPEC;
impl crate::RegisterSpec for IDVRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idvris::R`](R) reader structure"]
impl crate::Readable for IDVRIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idvris::W`](W) writer structure"]
impl crate::Writable for IDVRIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDVRIS to value 0"]
impl crate::Resettable for IDVRIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
