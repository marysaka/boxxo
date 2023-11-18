#[doc = "Register `NWDA2` reader"]
pub type R = crate::R<NWDA2_SPEC>;
#[doc = "Register `NWDA2` writer"]
pub type W = crate::W<NWDA2_SPEC>;
#[doc = "Field `CAN_NWDA2_NEWDAT` reader - New Data Bits"]
pub type CAN_NWDA2_NEWDAT_R = crate::FieldReader<u16>;
#[doc = "Field `CAN_NWDA2_NEWDAT` writer - New Data Bits"]
pub type CAN_NWDA2_NEWDAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - New Data Bits"]
    #[inline(always)]
    pub fn can_nwda2_newdat(&self) -> CAN_NWDA2_NEWDAT_R {
        CAN_NWDA2_NEWDAT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - New Data Bits"]
    #[inline(always)]
    #[must_use]
    pub fn can_nwda2_newdat(&mut self) -> CAN_NWDA2_NEWDAT_W<NWDA2_SPEC, 0> {
        CAN_NWDA2_NEWDAT_W::new(self)
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
#[doc = "CAN New Data 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nwda2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nwda2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NWDA2_SPEC;
impl crate::RegisterSpec for NWDA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nwda2::R`](R) reader structure"]
impl crate::Readable for NWDA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nwda2::W`](W) writer structure"]
impl crate::Writable for NWDA2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NWDA2 to value 0"]
impl crate::Resettable for NWDA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
