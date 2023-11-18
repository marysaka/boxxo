#[doc = "Register `USERREG0` reader"]
pub type R = crate::R<USERREG0_SPEC>;
#[doc = "Register `USERREG0` writer"]
pub type W = crate::W<USERREG0_SPEC>;
#[doc = "Field `FLASH_USERREG0_DATA` reader - User Data"]
pub type FLASH_USERREG0_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `FLASH_USERREG0_DATA` writer - User Data"]
pub type FLASH_USERREG0_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    pub fn flash_userreg0_data(&self) -> FLASH_USERREG0_DATA_R {
        FLASH_USERREG0_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    #[must_use]
    pub fn flash_userreg0_data(&mut self) -> FLASH_USERREG0_DATA_W<USERREG0_SPEC, 0> {
        FLASH_USERREG0_DATA_W::new(self)
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
#[doc = "User Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userreg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userreg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USERREG0_SPEC;
impl crate::RegisterSpec for USERREG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`userreg0::R`](R) reader structure"]
impl crate::Readable for USERREG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`userreg0::W`](W) writer structure"]
impl crate::Writable for USERREG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USERREG0 to value 0"]
impl crate::Resettable for USERREG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
