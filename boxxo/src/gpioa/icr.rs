#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `GPIO_ICR_GPIO` writer - GPIO Interrupt Clear"]
pub type GPIO_ICR_GPIO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl W {
    #[doc = "Bits 0:7 - GPIO Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_icr_gpio(&mut self) -> GPIO_ICR_GPIO_W<ICR_SPEC, 0> {
        GPIO_ICR_GPIO_W::new(self)
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
#[doc = "GPIO Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
