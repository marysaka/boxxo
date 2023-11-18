#[doc = "Register `USERREG1` reader"]
pub type R = crate::R<USERREG1_SPEC>;
#[doc = "Register `USERREG1` writer"]
pub type W = crate::W<USERREG1_SPEC>;
#[doc = "Field `FLASH_USERREG1_DATA` reader - User Data"]
pub type FLASH_USERREG1_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `FLASH_USERREG1_DATA` writer - User Data"]
pub type FLASH_USERREG1_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    pub fn flash_userreg1_data(&self) -> FLASH_USERREG1_DATA_R {
        FLASH_USERREG1_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    #[must_use]
    pub fn flash_userreg1_data(&mut self) -> FLASH_USERREG1_DATA_W<USERREG1_SPEC, 0> {
        FLASH_USERREG1_DATA_W::new(self)
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
#[doc = "User Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userreg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userreg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USERREG1_SPEC;
impl crate::RegisterSpec for USERREG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`userreg1::R`](R) reader structure"]
impl crate::Readable for USERREG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`userreg1::W`](W) writer structure"]
impl crate::Writable for USERREG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USERREG1 to value 0"]
impl crate::Resettable for USERREG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
