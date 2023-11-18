#[doc = "Register `PIOSCSTAT` reader"]
pub type R = crate::R<PIOSCSTAT_SPEC>;
#[doc = "Register `PIOSCSTAT` writer"]
pub type W = crate::W<PIOSCSTAT_SPEC>;
#[doc = "Field `SYSCTL_PIOSCSTAT_CT` reader - Calibration Trim Value"]
pub type SYSCTL_PIOSCSTAT_CT_R = crate::FieldReader;
#[doc = "Field `SYSCTL_PIOSCSTAT_CT` writer - Calibration Trim Value"]
pub type SYSCTL_PIOSCSTAT_CT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `SYSCTL_PIOSCSTAT_CR` reader - Calibration Result"]
pub type SYSCTL_PIOSCSTAT_CR_R = crate::FieldReader<SYSCTL_PIOSCSTAT_CR_A>;
#[doc = "Calibration Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_PIOSCSTAT_CR_A {
    #[doc = "0: Calibration has not been attempted"]
    SYSCTL_PIOSCSTAT_CRNONE = 0,
    #[doc = "1: The last calibration operation completed to meet 1% accuracy"]
    SYSCTL_PIOSCSTAT_CRPASS = 1,
    #[doc = "2: The last calibration operation failed to meet 1% accuracy"]
    SYSCTL_PIOSCSTAT_CRFAIL = 2,
}
impl From<SYSCTL_PIOSCSTAT_CR_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_PIOSCSTAT_CR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_PIOSCSTAT_CR_A {
    type Ux = u8;
}
impl SYSCTL_PIOSCSTAT_CR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_PIOSCSTAT_CR_A> {
        match self.bits {
            0 => Some(SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRNONE),
            1 => Some(SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRPASS),
            2 => Some(SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRFAIL),
            _ => None,
        }
    }
    #[doc = "Calibration has not been attempted"]
    #[inline(always)]
    pub fn is_sysctl_pioscstat_crnone(&self) -> bool {
        *self == SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRNONE
    }
    #[doc = "The last calibration operation completed to meet 1% accuracy"]
    #[inline(always)]
    pub fn is_sysctl_pioscstat_crpass(&self) -> bool {
        *self == SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRPASS
    }
    #[doc = "The last calibration operation failed to meet 1% accuracy"]
    #[inline(always)]
    pub fn is_sysctl_pioscstat_crfail(&self) -> bool {
        *self == SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRFAIL
    }
}
#[doc = "Field `SYSCTL_PIOSCSTAT_CR` writer - Calibration Result"]
pub type SYSCTL_PIOSCSTAT_CR_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, SYSCTL_PIOSCSTAT_CR_A>;
impl<'a, REG, const O: u8> SYSCTL_PIOSCSTAT_CR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Calibration has not been attempted"]
    #[inline(always)]
    pub fn sysctl_pioscstat_crnone(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRNONE)
    }
    #[doc = "The last calibration operation completed to meet 1% accuracy"]
    #[inline(always)]
    pub fn sysctl_pioscstat_crpass(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRPASS)
    }
    #[doc = "The last calibration operation failed to meet 1% accuracy"]
    #[inline(always)]
    pub fn sysctl_pioscstat_crfail(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRFAIL)
    }
}
#[doc = "Field `SYSCTL_PIOSCSTAT_DT` reader - Default Trim Value"]
pub type SYSCTL_PIOSCSTAT_DT_R = crate::FieldReader;
#[doc = "Field `SYSCTL_PIOSCSTAT_DT` writer - Default Trim Value"]
pub type SYSCTL_PIOSCSTAT_DT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Calibration Trim Value"]
    #[inline(always)]
    pub fn sysctl_pioscstat_ct(&self) -> SYSCTL_PIOSCSTAT_CT_R {
        SYSCTL_PIOSCSTAT_CT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Calibration Result"]
    #[inline(always)]
    pub fn sysctl_pioscstat_cr(&self) -> SYSCTL_PIOSCSTAT_CR_R {
        SYSCTL_PIOSCSTAT_CR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:22 - Default Trim Value"]
    #[inline(always)]
    pub fn sysctl_pioscstat_dt(&self) -> SYSCTL_PIOSCSTAT_DT_R {
        SYSCTL_PIOSCSTAT_DT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pioscstat_ct(&mut self) -> SYSCTL_PIOSCSTAT_CT_W<PIOSCSTAT_SPEC, 0> {
        SYSCTL_PIOSCSTAT_CT_W::new(self)
    }
    #[doc = "Bits 8:9 - Calibration Result"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pioscstat_cr(&mut self) -> SYSCTL_PIOSCSTAT_CR_W<PIOSCSTAT_SPEC, 8> {
        SYSCTL_PIOSCSTAT_CR_W::new(self)
    }
    #[doc = "Bits 16:22 - Default Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pioscstat_dt(&mut self) -> SYSCTL_PIOSCSTAT_DT_W<PIOSCSTAT_SPEC, 16> {
        SYSCTL_PIOSCSTAT_DT_W::new(self)
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
#[doc = "Precision Internal Oscillator Statistics\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pioscstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pioscstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIOSCSTAT_SPEC;
impl crate::RegisterSpec for PIOSCSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pioscstat::R`](R) reader structure"]
impl crate::Readable for PIOSCSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pioscstat::W`](W) writer structure"]
impl crate::Writable for PIOSCSTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIOSCSTAT to value 0"]
impl crate::Resettable for PIOSCSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
