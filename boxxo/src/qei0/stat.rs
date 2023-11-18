#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `QEI_STAT_ERROR` reader - Error Detected"]
pub type QEI_STAT_ERROR_R = crate::BitReader;
#[doc = "Field `QEI_STAT_ERROR` writer - Error Detected"]
pub type QEI_STAT_ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_STAT_DIRECTION` reader - Direction of Rotation"]
pub type QEI_STAT_DIRECTION_R = crate::BitReader;
#[doc = "Field `QEI_STAT_DIRECTION` writer - Direction of Rotation"]
pub type QEI_STAT_DIRECTION_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Error Detected"]
    #[inline(always)]
    pub fn qei_stat_error(&self) -> QEI_STAT_ERROR_R {
        QEI_STAT_ERROR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction of Rotation"]
    #[inline(always)]
    pub fn qei_stat_direction(&self) -> QEI_STAT_DIRECTION_R {
        QEI_STAT_DIRECTION_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Detected"]
    #[inline(always)]
    #[must_use]
    pub fn qei_stat_error(&mut self) -> QEI_STAT_ERROR_W<STAT_SPEC, 0> {
        QEI_STAT_ERROR_W::new(self)
    }
    #[doc = "Bit 1 - Direction of Rotation"]
    #[inline(always)]
    #[must_use]
    pub fn qei_stat_direction(&mut self) -> QEI_STAT_DIRECTION_W<STAT_SPEC, 1> {
        QEI_STAT_DIRECTION_W::new(self)
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
#[doc = "QEI Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
