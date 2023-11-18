#[doc = "Register `_9BITADDR` reader"]
pub type R = crate::R<_9BITADDR_SPEC>;
#[doc = "Register `_9BITADDR` writer"]
pub type W = crate::W<_9BITADDR_SPEC>;
#[doc = "Field `UART_9BITADDR_ADDR` reader - Self Address for 9-Bit Mode"]
pub type UART_9BITADDR_ADDR_R = crate::FieldReader;
#[doc = "Field `UART_9BITADDR_ADDR` writer - Self Address for 9-Bit Mode"]
pub type UART_9BITADDR_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `UART_9BITADDR_9BITEN` reader - Enable 9-Bit Mode"]
pub type UART_9BITADDR_9BITEN_R = crate::BitReader;
#[doc = "Field `UART_9BITADDR_9BITEN` writer - Enable 9-Bit Mode"]
pub type UART_9BITADDR_9BITEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - Self Address for 9-Bit Mode"]
    #[inline(always)]
    pub fn uart_9bitaddr_addr(&self) -> UART_9BITADDR_ADDR_R {
        UART_9BITADDR_ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Enable 9-Bit Mode"]
    #[inline(always)]
    pub fn uart_9bitaddr_9biten(&self) -> UART_9BITADDR_9BITEN_R {
        UART_9BITADDR_9BITEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self Address for 9-Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn uart_9bitaddr_addr(&mut self) -> UART_9BITADDR_ADDR_W<_9BITADDR_SPEC, 0> {
        UART_9BITADDR_ADDR_W::new(self)
    }
    #[doc = "Bit 15 - Enable 9-Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn uart_9bitaddr_9biten(&mut self) -> UART_9BITADDR_9BITEN_W<_9BITADDR_SPEC, 15> {
        UART_9BITADDR_9BITEN_W::new(self)
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
#[doc = "UART 9-Bit Self Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_9bitaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_9bitaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _9BITADDR_SPEC;
impl crate::RegisterSpec for _9BITADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_9bitaddr::R`](R) reader structure"]
impl crate::Readable for _9BITADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_9bitaddr::W`](W) writer structure"]
impl crate::Writable for _9BITADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _9BITADDR to value 0"]
impl crate::Resettable for _9BITADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
