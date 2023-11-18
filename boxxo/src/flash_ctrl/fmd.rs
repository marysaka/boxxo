#[doc = "Register `FMD` reader"]
pub type R = crate::R<FMD_SPEC>;
#[doc = "Register `FMD` writer"]
pub type W = crate::W<FMD_SPEC>;
#[doc = "Field `FLASH_FMD_DATA` reader - Data Value"]
pub type FLASH_FMD_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `FLASH_FMD_DATA` writer - Data Value"]
pub type FLASH_FMD_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Value"]
    #[inline(always)]
    pub fn flash_fmd_data(&self) -> FLASH_FMD_DATA_R {
        FLASH_FMD_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Value"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fmd_data(&mut self) -> FLASH_FMD_DATA_W<FMD_SPEC, 0> {
        FLASH_FMD_DATA_W::new(self)
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
#[doc = "Flash Memory Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMD_SPEC;
impl crate::RegisterSpec for FMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmd::R`](R) reader structure"]
impl crate::Readable for FMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmd::W`](W) writer structure"]
impl crate::Writable for FMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMD to value 0"]
impl crate::Resettable for FMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
