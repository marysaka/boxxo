#[doc = "Register `FMC2` reader"]
pub type R = crate::R<FMC2_SPEC>;
#[doc = "Register `FMC2` writer"]
pub type W = crate::W<FMC2_SPEC>;
#[doc = "Field `FLASH_FMC2_WRBUF` reader - Buffered Flash Memory Write"]
pub type FLASH_FMC2_WRBUF_R = crate::BitReader;
#[doc = "Field `FLASH_FMC2_WRBUF` writer - Buffered Flash Memory Write"]
pub type FLASH_FMC2_WRBUF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Buffered Flash Memory Write"]
    #[inline(always)]
    pub fn flash_fmc2_wrbuf(&self) -> FLASH_FMC2_WRBUF_R {
        FLASH_FMC2_WRBUF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffered Flash Memory Write"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fmc2_wrbuf(&mut self) -> FLASH_FMC2_WRBUF_W<FMC2_SPEC, 0> {
        FLASH_FMC2_WRBUF_W::new(self)
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
#[doc = "Flash Memory Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC2_SPEC;
impl crate::RegisterSpec for FMC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc2::R`](R) reader structure"]
impl crate::Readable for FMC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmc2::W`](W) writer structure"]
impl crate::Writable for FMC2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC2 to value 0"]
impl crate::Resettable for FMC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
