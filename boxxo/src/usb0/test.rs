#[doc = "Register `TEST` reader"]
pub type R = crate::R<TEST_SPEC>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TEST_SPEC>;
#[doc = "Field `USB_TEST_FORCEFS` reader - Force Full-Speed Mode"]
pub type USB_TEST_FORCEFS_R = crate::BitReader;
#[doc = "Field `USB_TEST_FORCEFS` writer - Force Full-Speed Mode"]
pub type USB_TEST_FORCEFS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TEST_FIFOACC` reader - FIFO Access"]
pub type USB_TEST_FIFOACC_R = crate::BitReader;
#[doc = "Field `USB_TEST_FIFOACC` writer - FIFO Access"]
pub type USB_TEST_FIFOACC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_TEST_FORCEH` reader - Force Host Mode"]
pub type USB_TEST_FORCEH_R = crate::BitReader;
#[doc = "Field `USB_TEST_FORCEH` writer - Force Host Mode"]
pub type USB_TEST_FORCEH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 5 - Force Full-Speed Mode"]
    #[inline(always)]
    pub fn usb_test_forcefs(&self) -> USB_TEST_FORCEFS_R {
        USB_TEST_FORCEFS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO Access"]
    #[inline(always)]
    pub fn usb_test_fifoacc(&self) -> USB_TEST_FIFOACC_R {
        USB_TEST_FIFOACC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Host Mode"]
    #[inline(always)]
    pub fn usb_test_forceh(&self) -> USB_TEST_FORCEH_R {
        USB_TEST_FORCEH_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Force Full-Speed Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb_test_forcefs(&mut self) -> USB_TEST_FORCEFS_W<TEST_SPEC, 5> {
        USB_TEST_FORCEFS_W::new(self)
    }
    #[doc = "Bit 6 - FIFO Access"]
    #[inline(always)]
    #[must_use]
    pub fn usb_test_fifoacc(&mut self) -> USB_TEST_FIFOACC_W<TEST_SPEC, 6> {
        USB_TEST_FIFOACC_W::new(self)
    }
    #[doc = "Bit 7 - Force Host Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb_test_forceh(&mut self) -> USB_TEST_FORCEH_W<TEST_SPEC, 7> {
        USB_TEST_FORCEH_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Test Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
