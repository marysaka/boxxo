#[doc = "Register `ROMSWMAP` reader"]
pub type R = crate::R<FLASH_CTRL_ALT_ROMSWMAP_SPEC>;
#[doc = "Register `ROMSWMAP` writer"]
pub type W = crate::W<FLASH_CTRL_ALT_ROMSWMAP_SPEC>;
#[doc = "Field `FLASH_ROMSWMAP_SAFERTOS` reader - SafeRTOS Present"]
pub type FLASH_ROMSWMAP_SAFERTOS_R = crate::BitReader;
#[doc = "Field `FLASH_ROMSWMAP_SAFERTOS` writer - SafeRTOS Present"]
pub type FLASH_ROMSWMAP_SAFERTOS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SafeRTOS Present"]
    #[inline(always)]
    pub fn flash_romswmap_safertos(&self) -> FLASH_ROMSWMAP_SAFERTOS_R {
        FLASH_ROMSWMAP_SAFERTOS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SafeRTOS Present"]
    #[inline(always)]
    #[must_use]
    pub fn flash_romswmap_safertos(
        &mut self,
    ) -> FLASH_ROMSWMAP_SAFERTOS_W<FLASH_CTRL_ALT_ROMSWMAP_SPEC, 0> {
        FLASH_ROMSWMAP_SAFERTOS_W::new(self)
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
#[doc = "ROM Software Map\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ctrl_alt_romswmap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ctrl_alt_romswmap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_CTRL_ALT_ROMSWMAP_SPEC;
impl crate::RegisterSpec for FLASH_CTRL_ALT_ROMSWMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_ctrl_alt_romswmap::R`](R) reader structure"]
impl crate::Readable for FLASH_CTRL_ALT_ROMSWMAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_ctrl_alt_romswmap::W`](W) writer structure"]
impl crate::Writable for FLASH_CTRL_ALT_ROMSWMAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROMSWMAP to value 0"]
impl crate::Resettable for FLASH_CTRL_ALT_ROMSWMAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
