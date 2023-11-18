#[doc = "Register `ACSTAT0` reader"]
pub type R = crate::R<ACSTAT0_SPEC>;
#[doc = "Register `ACSTAT0` writer"]
pub type W = crate::W<ACSTAT0_SPEC>;
#[doc = "Field `COMP_ACSTAT0_OVAL` reader - Comparator Output Value"]
pub type COMP_ACSTAT0_OVAL_R = crate::BitReader;
#[doc = "Field `COMP_ACSTAT0_OVAL` writer - Comparator Output Value"]
pub type COMP_ACSTAT0_OVAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Comparator Output Value"]
    #[inline(always)]
    pub fn comp_acstat0_oval(&self) -> COMP_ACSTAT0_OVAL_R {
        COMP_ACSTAT0_OVAL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Comparator Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn comp_acstat0_oval(&mut self) -> COMP_ACSTAT0_OVAL_W<ACSTAT0_SPEC, 1> {
        COMP_ACSTAT0_OVAL_W::new(self)
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
#[doc = "Analog Comparator Status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acstat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acstat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACSTAT0_SPEC;
impl crate::RegisterSpec for ACSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acstat0::R`](R) reader structure"]
impl crate::Readable for ACSTAT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acstat0::W`](W) writer structure"]
impl crate::Writable for ACSTAT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSTAT0 to value 0"]
impl crate::Resettable for ACSTAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
