#[doc = "Register `PP` reader"]
pub type R = crate::R<PP_SPEC>;
#[doc = "Register `PP` writer"]
pub type W = crate::W<PP_SPEC>;
#[doc = "Field `UART_PP_SC` reader - Smart Card Support"]
pub type UART_PP_SC_R = crate::BitReader;
#[doc = "Field `UART_PP_SC` writer - Smart Card Support"]
pub type UART_PP_SC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_PP_NB` reader - 9-Bit Support"]
pub type UART_PP_NB_R = crate::BitReader;
#[doc = "Field `UART_PP_NB` writer - 9-Bit Support"]
pub type UART_PP_NB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Smart Card Support"]
    #[inline(always)]
    pub fn uart_pp_sc(&self) -> UART_PP_SC_R {
        UART_PP_SC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 9-Bit Support"]
    #[inline(always)]
    pub fn uart_pp_nb(&self) -> UART_PP_NB_R {
        UART_PP_NB_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Smart Card Support"]
    #[inline(always)]
    #[must_use]
    pub fn uart_pp_sc(&mut self) -> UART_PP_SC_W<PP_SPEC, 0> {
        UART_PP_SC_W::new(self)
    }
    #[doc = "Bit 1 - 9-Bit Support"]
    #[inline(always)]
    #[must_use]
    pub fn uart_pp_nb(&mut self) -> UART_PP_NB_W<PP_SPEC, 1> {
        UART_PP_NB_W::new(self)
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
#[doc = "UART Peripheral Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pp::R`](R) reader structure"]
impl crate::Readable for PP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pp::W`](W) writer structure"]
impl crate::Writable for PP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
