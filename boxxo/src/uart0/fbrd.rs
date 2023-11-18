#[doc = "Register `FBRD` reader"]
pub type R = crate::R<FBRD_SPEC>;
#[doc = "Register `FBRD` writer"]
pub type W = crate::W<FBRD_SPEC>;
#[doc = "Field `UART_FBRD_DIVFRAC` reader - Fractional Baud-Rate Divisor"]
pub type UART_FBRD_DIVFRAC_R = crate::FieldReader;
#[doc = "Field `UART_FBRD_DIVFRAC` writer - Fractional Baud-Rate Divisor"]
pub type UART_FBRD_DIVFRAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Fractional Baud-Rate Divisor"]
    #[inline(always)]
    pub fn uart_fbrd_divfrac(&self) -> UART_FBRD_DIVFRAC_R {
        UART_FBRD_DIVFRAC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fractional Baud-Rate Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn uart_fbrd_divfrac(&mut self) -> UART_FBRD_DIVFRAC_W<FBRD_SPEC, 0> {
        UART_FBRD_DIVFRAC_W::new(self)
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
#[doc = "UART Fractional Baud-Rate Divisor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FBRD_SPEC;
impl crate::RegisterSpec for FBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbrd::R`](R) reader structure"]
impl crate::Readable for FBRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fbrd::W`](W) writer structure"]
impl crate::Writable for FBRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBRD to value 0"]
impl crate::Resettable for FBRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
