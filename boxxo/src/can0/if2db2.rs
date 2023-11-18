#[doc = "Register `IF2DB2` reader"]
pub type R = crate::R<IF2DB2_SPEC>;
#[doc = "Register `IF2DB2` writer"]
pub type W = crate::W<IF2DB2_SPEC>;
#[doc = "Field `CAN_IF2DB2_DATA` reader - Data"]
pub type CAN_IF2DB2_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `CAN_IF2DB2_DATA` writer - Data"]
pub type CAN_IF2DB2_DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Data"]
    #[inline(always)]
    pub fn can_if2db2_data(&self) -> CAN_IF2DB2_DATA_R {
        CAN_IF2DB2_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data"]
    #[inline(always)]
    #[must_use]
    pub fn can_if2db2_data(&mut self) -> CAN_IF2DB2_DATA_W<IF2DB2_SPEC, 0> {
        CAN_IF2DB2_DATA_W::new(self)
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
#[doc = "CAN IF2 Data B2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if2db2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if2db2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF2DB2_SPEC;
impl crate::RegisterSpec for IF2DB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if2db2::R`](R) reader structure"]
impl crate::Readable for IF2DB2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`if2db2::W`](W) writer structure"]
impl crate::Writable for IF2DB2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF2DB2 to value 0"]
impl crate::Resettable for IF2DB2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
