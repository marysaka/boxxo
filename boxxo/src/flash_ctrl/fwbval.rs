#[doc = "Register `FWBVAL` reader"]
pub type R = crate::R<FWBVAL_SPEC>;
#[doc = "Register `FWBVAL` writer"]
pub type W = crate::W<FWBVAL_SPEC>;
#[doc = "Field `FLASH_FWBVAL_FWB` reader - Flash Memory Write Buffer"]
pub type FLASH_FWBVAL_FWB_R = crate::FieldReader<u32>;
#[doc = "Field `FLASH_FWBVAL_FWB` writer - Flash Memory Write Buffer"]
pub type FLASH_FWBVAL_FWB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Flash Memory Write Buffer"]
    #[inline(always)]
    pub fn flash_fwbval_fwb(&self) -> FLASH_FWBVAL_FWB_R {
        FLASH_FWBVAL_FWB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Flash Memory Write Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fwbval_fwb(&mut self) -> FLASH_FWBVAL_FWB_W<FWBVAL_SPEC, 0> {
        FLASH_FWBVAL_FWB_W::new(self)
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
#[doc = "Flash Write Buffer Valid\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwbval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwbval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FWBVAL_SPEC;
impl crate::RegisterSpec for FWBVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwbval::R`](R) reader structure"]
impl crate::Readable for FWBVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fwbval::W`](W) writer structure"]
impl crate::Writable for FWBVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FWBVAL to value 0"]
impl crate::Resettable for FWBVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
