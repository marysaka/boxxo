#[doc = "Register `FMA` reader"]
pub type R = crate::R<FMA_SPEC>;
#[doc = "Register `FMA` writer"]
pub type W = crate::W<FMA_SPEC>;
#[doc = "Field `FLASH_FMA_OFFSET` reader - Address Offset"]
pub type FLASH_FMA_OFFSET_R = crate::FieldReader<u32>;
#[doc = "Field `FLASH_FMA_OFFSET` writer - Address Offset"]
pub type FLASH_FMA_OFFSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 18, O, u32>;
impl R {
    #[doc = "Bits 0:17 - Address Offset"]
    #[inline(always)]
    pub fn flash_fma_offset(&self) -> FLASH_FMA_OFFSET_R {
        FLASH_FMA_OFFSET_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Address Offset"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fma_offset(&mut self) -> FLASH_FMA_OFFSET_W<FMA_SPEC, 0> {
        FLASH_FMA_OFFSET_W::new(self)
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
#[doc = "Flash Memory Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMA_SPEC;
impl crate::RegisterSpec for FMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fma::R`](R) reader structure"]
impl crate::Readable for FMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fma::W`](W) writer structure"]
impl crate::Writable for FMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMA to value 0"]
impl crate::Resettable for FMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
