#[doc = "Register `RMCTL` reader"]
pub type R = crate::R<RMCTL_SPEC>;
#[doc = "Register `RMCTL` writer"]
pub type W = crate::W<RMCTL_SPEC>;
#[doc = "Field `FLASH_RMCTL_BA` reader - Boot Alias"]
pub type FLASH_RMCTL_BA_R = crate::BitReader;
#[doc = "Field `FLASH_RMCTL_BA` writer - Boot Alias"]
pub type FLASH_RMCTL_BA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Boot Alias"]
    #[inline(always)]
    pub fn flash_rmctl_ba(&self) -> FLASH_RMCTL_BA_R {
        FLASH_RMCTL_BA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Boot Alias"]
    #[inline(always)]
    #[must_use]
    pub fn flash_rmctl_ba(&mut self) -> FLASH_RMCTL_BA_W<RMCTL_SPEC, 0> {
        FLASH_RMCTL_BA_W::new(self)
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
#[doc = "ROM Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMCTL_SPEC;
impl crate::RegisterSpec for RMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmctl::R`](R) reader structure"]
impl crate::Readable for RMCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rmctl::W`](W) writer structure"]
impl crate::Writable for RMCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMCTL to value 0"]
impl crate::Resettable for RMCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
