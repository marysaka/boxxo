#[doc = "Register `ACMIS` reader"]
pub type R = crate::R<ACMIS_SPEC>;
#[doc = "Register `ACMIS` writer"]
pub type W = crate::W<ACMIS_SPEC>;
#[doc = "Field `COMP_ACMIS_IN0` reader - Comparator 0 Masked Interrupt Status"]
pub type COMP_ACMIS_IN0_R = crate::BitReader;
#[doc = "Field `COMP_ACMIS_IN0` writer - Comparator 0 Masked Interrupt Status"]
pub type COMP_ACMIS_IN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP_ACMIS_IN1` reader - Comparator 1 Masked Interrupt Status"]
pub type COMP_ACMIS_IN1_R = crate::BitReader;
#[doc = "Field `COMP_ACMIS_IN1` writer - Comparator 1 Masked Interrupt Status"]
pub type COMP_ACMIS_IN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Masked Interrupt Status"]
    #[inline(always)]
    pub fn comp_acmis_in0(&self) -> COMP_ACMIS_IN0_R {
        COMP_ACMIS_IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Masked Interrupt Status"]
    #[inline(always)]
    pub fn comp_acmis_in1(&self) -> COMP_ACMIS_IN1_R {
        COMP_ACMIS_IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn comp_acmis_in0(&mut self) -> COMP_ACMIS_IN0_W<ACMIS_SPEC, 0> {
        COMP_ACMIS_IN0_W::new(self)
    }
    #[doc = "Bit 1 - Comparator 1 Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn comp_acmis_in1(&mut self) -> COMP_ACMIS_IN1_W<ACMIS_SPEC, 1> {
        COMP_ACMIS_IN1_W::new(self)
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
#[doc = "Analog Comparator Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acmis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acmis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACMIS_SPEC;
impl crate::RegisterSpec for ACMIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acmis::R`](R) reader structure"]
impl crate::Readable for ACMIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acmis::W`](W) writer structure"]
impl crate::Writable for ACMIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACMIS to value 0"]
impl crate::Resettable for ACMIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
