#[doc = "Register `_9BITAMASK` reader"]
pub type R = crate::R<_9BITAMASK_SPEC>;
#[doc = "Register `_9BITAMASK` writer"]
pub type W = crate::W<_9BITAMASK_SPEC>;
#[doc = "Field `UART_9BITAMASK_MASK` reader - Self Address Mask for 9-Bit Mode"]
pub type UART_9BITAMASK_MASK_R = crate::FieldReader;
#[doc = "Field `UART_9BITAMASK_MASK` writer - Self Address Mask for 9-Bit Mode"]
pub type UART_9BITAMASK_MASK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode"]
    #[inline(always)]
    pub fn uart_9bitamask_mask(&self) -> UART_9BITAMASK_MASK_R {
        UART_9BITAMASK_MASK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn uart_9bitamask_mask(&mut self) -> UART_9BITAMASK_MASK_W<_9BITAMASK_SPEC, 0> {
        UART_9BITAMASK_MASK_W::new(self)
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
#[doc = "UART 9-Bit Self Address Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_9bitamask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_9bitamask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _9BITAMASK_SPEC;
impl crate::RegisterSpec for _9BITAMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_9bitamask::R`](R) reader structure"]
impl crate::Readable for _9BITAMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_9bitamask::W`](W) writer structure"]
impl crate::Writable for _9BITAMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _9BITAMASK to value 0"]
impl crate::Resettable for _9BITAMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
