#[doc = "Register `PP` reader"]
pub type R = crate::R<PP_SPEC>;
#[doc = "Register `PP` writer"]
pub type W = crate::W<PP_SPEC>;
#[doc = "Field `COMP_PP_CMP0` reader - Comparator 0 Present"]
pub type COMP_PP_CMP0_R = crate::BitReader;
#[doc = "Field `COMP_PP_CMP0` writer - Comparator 0 Present"]
pub type COMP_PP_CMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP_PP_CMP1` reader - Comparator 1 Present"]
pub type COMP_PP_CMP1_R = crate::BitReader;
#[doc = "Field `COMP_PP_CMP1` writer - Comparator 1 Present"]
pub type COMP_PP_CMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP_PP_C0O` reader - Comparator Output 0 Present"]
pub type COMP_PP_C0O_R = crate::BitReader;
#[doc = "Field `COMP_PP_C0O` writer - Comparator Output 0 Present"]
pub type COMP_PP_C0O_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP_PP_C1O` reader - Comparator Output 1 Present"]
pub type COMP_PP_C1O_R = crate::BitReader;
#[doc = "Field `COMP_PP_C1O` writer - Comparator Output 1 Present"]
pub type COMP_PP_C1O_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Present"]
    #[inline(always)]
    pub fn comp_pp_cmp0(&self) -> COMP_PP_CMP0_R {
        COMP_PP_CMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Present"]
    #[inline(always)]
    pub fn comp_pp_cmp1(&self) -> COMP_PP_CMP1_R {
        COMP_PP_CMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Comparator Output 0 Present"]
    #[inline(always)]
    pub fn comp_pp_c0o(&self) -> COMP_PP_C0O_R {
        COMP_PP_C0O_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Comparator Output 1 Present"]
    #[inline(always)]
    pub fn comp_pp_c1o(&self) -> COMP_PP_C1O_R {
        COMP_PP_C1O_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn comp_pp_cmp0(&mut self) -> COMP_PP_CMP0_W<PP_SPEC, 0> {
        COMP_PP_CMP0_W::new(self)
    }
    #[doc = "Bit 1 - Comparator 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn comp_pp_cmp1(&mut self) -> COMP_PP_CMP1_W<PP_SPEC, 1> {
        COMP_PP_CMP1_W::new(self)
    }
    #[doc = "Bit 16 - Comparator Output 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn comp_pp_c0o(&mut self) -> COMP_PP_C0O_W<PP_SPEC, 16> {
        COMP_PP_C0O_W::new(self)
    }
    #[doc = "Bit 17 - Comparator Output 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn comp_pp_c1o(&mut self) -> COMP_PP_C1O_W<PP_SPEC, 17> {
        COMP_PP_C1O_W::new(self)
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
#[doc = "Analog Comparator Peripheral Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pp::R`](R) reader structure"]
impl crate::Readable for PP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pp::W`](W) writer structure"]
impl crate::Writable for PP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
