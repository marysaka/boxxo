#[doc = "Register `FMC` reader"]
pub type R = crate::R<FMC_SPEC>;
#[doc = "Register `FMC` writer"]
pub type W = crate::W<FMC_SPEC>;
#[doc = "Field `FLASH_FMC_WRITE` reader - Write a Word into Flash Memory"]
pub type FLASH_FMC_WRITE_R = crate::BitReader;
#[doc = "Field `FLASH_FMC_WRITE` writer - Write a Word into Flash Memory"]
pub type FLASH_FMC_WRITE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FMC_ERASE` reader - Erase a Page of Flash Memory"]
pub type FLASH_FMC_ERASE_R = crate::BitReader;
#[doc = "Field `FLASH_FMC_ERASE` writer - Erase a Page of Flash Memory"]
pub type FLASH_FMC_ERASE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FMC_MERASE` reader - Mass Erase Flash Memory"]
pub type FLASH_FMC_MERASE_R = crate::BitReader;
#[doc = "Field `FLASH_FMC_MERASE` writer - Mass Erase Flash Memory"]
pub type FLASH_FMC_MERASE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FMC_COMT` reader - Commit Register Value"]
pub type FLASH_FMC_COMT_R = crate::BitReader;
#[doc = "Field `FLASH_FMC_COMT` writer - Commit Register Value"]
pub type FLASH_FMC_COMT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASH_FMC_WRKEY` reader - FLASH write key"]
pub type FLASH_FMC_WRKEY_R = crate::FieldReader<u16>;
#[doc = "Field `FLASH_FMC_WRKEY` writer - FLASH write key"]
pub type FLASH_FMC_WRKEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
impl R {
    #[doc = "Bit 0 - Write a Word into Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_write(&self) -> FLASH_FMC_WRITE_R {
        FLASH_FMC_WRITE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Erase a Page of Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_erase(&self) -> FLASH_FMC_ERASE_R {
        FLASH_FMC_ERASE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mass Erase Flash Memory"]
    #[inline(always)]
    pub fn flash_fmc_merase(&self) -> FLASH_FMC_MERASE_R {
        FLASH_FMC_MERASE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Commit Register Value"]
    #[inline(always)]
    pub fn flash_fmc_comt(&self) -> FLASH_FMC_COMT_R {
        FLASH_FMC_COMT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 17:31 - FLASH write key"]
    #[inline(always)]
    pub fn flash_fmc_wrkey(&self) -> FLASH_FMC_WRKEY_R {
        FLASH_FMC_WRKEY_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Write a Word into Flash Memory"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fmc_write(&mut self) -> FLASH_FMC_WRITE_W<FMC_SPEC, 0> {
        FLASH_FMC_WRITE_W::new(self)
    }
    #[doc = "Bit 1 - Erase a Page of Flash Memory"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fmc_erase(&mut self) -> FLASH_FMC_ERASE_W<FMC_SPEC, 1> {
        FLASH_FMC_ERASE_W::new(self)
    }
    #[doc = "Bit 2 - Mass Erase Flash Memory"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fmc_merase(&mut self) -> FLASH_FMC_MERASE_W<FMC_SPEC, 2> {
        FLASH_FMC_MERASE_W::new(self)
    }
    #[doc = "Bit 3 - Commit Register Value"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fmc_comt(&mut self) -> FLASH_FMC_COMT_W<FMC_SPEC, 3> {
        FLASH_FMC_COMT_W::new(self)
    }
    #[doc = "Bits 17:31 - FLASH write key"]
    #[inline(always)]
    #[must_use]
    pub fn flash_fmc_wrkey(&mut self) -> FLASH_FMC_WRKEY_W<FMC_SPEC, 17> {
        FLASH_FMC_WRKEY_W::new(self)
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
#[doc = "Flash Memory Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_SPEC;
impl crate::RegisterSpec for FMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc::R`](R) reader structure"]
impl crate::Readable for FMC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmc::W`](W) writer structure"]
impl crate::Writable for FMC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC to value 0"]
impl crate::Resettable for FMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
