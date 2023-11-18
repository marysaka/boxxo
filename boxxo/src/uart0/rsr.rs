#[doc = "Register `RSR` reader"]
pub type R = crate::R<RSR_SPEC>;
#[doc = "Register `RSR` writer"]
pub type W = crate::W<RSR_SPEC>;
#[doc = "Field `UART_RSR_FE` reader - UART Framing Error"]
pub type UART_RSR_FE_R = crate::BitReader;
#[doc = "Field `UART_RSR_FE` writer - UART Framing Error"]
pub type UART_RSR_FE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_RSR_PE` reader - UART Parity Error"]
pub type UART_RSR_PE_R = crate::BitReader;
#[doc = "Field `UART_RSR_PE` writer - UART Parity Error"]
pub type UART_RSR_PE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_RSR_BE` reader - UART Break Error"]
pub type UART_RSR_BE_R = crate::BitReader;
#[doc = "Field `UART_RSR_BE` writer - UART Break Error"]
pub type UART_RSR_BE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART_RSR_OE` reader - UART Overrun Error"]
pub type UART_RSR_OE_R = crate::BitReader;
#[doc = "Field `UART_RSR_OE` writer - UART Overrun Error"]
pub type UART_RSR_OE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - UART Framing Error"]
    #[inline(always)]
    pub fn uart_rsr_fe(&self) -> UART_RSR_FE_R {
        UART_RSR_FE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Parity Error"]
    #[inline(always)]
    pub fn uart_rsr_pe(&self) -> UART_RSR_PE_R {
        UART_RSR_PE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Break Error"]
    #[inline(always)]
    pub fn uart_rsr_be(&self) -> UART_RSR_BE_R {
        UART_RSR_BE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Overrun Error"]
    #[inline(always)]
    pub fn uart_rsr_oe(&self) -> UART_RSR_OE_R {
        UART_RSR_OE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Framing Error"]
    #[inline(always)]
    #[must_use]
    pub fn uart_rsr_fe(&mut self) -> UART_RSR_FE_W<RSR_SPEC, 0> {
        UART_RSR_FE_W::new(self)
    }
    #[doc = "Bit 1 - UART Parity Error"]
    #[inline(always)]
    #[must_use]
    pub fn uart_rsr_pe(&mut self) -> UART_RSR_PE_W<RSR_SPEC, 1> {
        UART_RSR_PE_W::new(self)
    }
    #[doc = "Bit 2 - UART Break Error"]
    #[inline(always)]
    #[must_use]
    pub fn uart_rsr_be(&mut self) -> UART_RSR_BE_W<RSR_SPEC, 2> {
        UART_RSR_BE_W::new(self)
    }
    #[doc = "Bit 3 - UART Overrun Error"]
    #[inline(always)]
    #[must_use]
    pub fn uart_rsr_oe(&mut self) -> UART_RSR_OE_W<RSR_SPEC, 3> {
        UART_RSR_OE_W::new(self)
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
#[doc = "UART Receive Status/Error Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSR_SPEC;
impl crate::RegisterSpec for RSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsr::R`](R) reader structure"]
impl crate::Readable for RSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rsr::W`](W) writer structure"]
impl crate::Writable for RSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSR to value 0"]
impl crate::Resettable for RSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
