#[doc = "Register `ILPR` reader"]
pub type R = crate::R<ILPR_SPEC>;
#[doc = "Register `ILPR` writer"]
pub type W = crate::W<ILPR_SPEC>;
#[doc = "Field `UART_ILPR_ILPDVSR` reader - IrDA Low-Power Divisor"]
pub type UART_ILPR_ILPDVSR_R = crate::FieldReader;
#[doc = "Field `UART_ILPR_ILPDVSR` writer - IrDA Low-Power Divisor"]
pub type UART_ILPR_ILPDVSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - IrDA Low-Power Divisor"]
    #[inline(always)]
    pub fn uart_ilpr_ilpdvsr(&self) -> UART_ILPR_ILPDVSR_R {
        UART_ILPR_ILPDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IrDA Low-Power Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn uart_ilpr_ilpdvsr(&mut self) -> UART_ILPR_ILPDVSR_W<ILPR_SPEC, 0> {
        UART_ILPR_ILPDVSR_W::new(self)
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
#[doc = "UART IrDA Low-Power Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ilpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ilpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ILPR_SPEC;
impl crate::RegisterSpec for ILPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ilpr::R`](R) reader structure"]
impl crate::Readable for ILPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ilpr::W`](W) writer structure"]
impl crate::Writable for ILPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ILPR to value 0"]
impl crate::Resettable for ILPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
