#[doc = "Register `IF2MSK2` reader"]
pub type R = crate::R<IF2MSK2_SPEC>;
#[doc = "Register `IF2MSK2` writer"]
pub type W = crate::W<IF2MSK2_SPEC>;
#[doc = "Field `CAN_IF2MSK2_IDMSK` reader - Identifier Mask"]
pub type CAN_IF2MSK2_IDMSK_R = crate::FieldReader<u16>;
#[doc = "Field `CAN_IF2MSK2_IDMSK` writer - Identifier Mask"]
pub type CAN_IF2MSK2_IDMSK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
#[doc = "Field `CAN_IF2MSK2_MDIR` reader - Mask Message Direction"]
pub type CAN_IF2MSK2_MDIR_R = crate::BitReader;
#[doc = "Field `CAN_IF2MSK2_MDIR` writer - Mask Message Direction"]
pub type CAN_IF2MSK2_MDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN_IF2MSK2_MXTD` reader - Mask Extended Identifier"]
pub type CAN_IF2MSK2_MXTD_R = crate::BitReader;
#[doc = "Field `CAN_IF2MSK2_MXTD` writer - Mask Extended Identifier"]
pub type CAN_IF2MSK2_MXTD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:12 - Identifier Mask"]
    #[inline(always)]
    pub fn can_if2msk2_idmsk(&self) -> CAN_IF2MSK2_IDMSK_R {
        CAN_IF2MSK2_IDMSK_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 14 - Mask Message Direction"]
    #[inline(always)]
    pub fn can_if2msk2_mdir(&self) -> CAN_IF2MSK2_MDIR_R {
        CAN_IF2MSK2_MDIR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Mask Extended Identifier"]
    #[inline(always)]
    pub fn can_if2msk2_mxtd(&self) -> CAN_IF2MSK2_MXTD_R {
        CAN_IF2MSK2_MXTD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Identifier Mask"]
    #[inline(always)]
    #[must_use]
    pub fn can_if2msk2_idmsk(&mut self) -> CAN_IF2MSK2_IDMSK_W<IF2MSK2_SPEC, 0> {
        CAN_IF2MSK2_IDMSK_W::new(self)
    }
    #[doc = "Bit 14 - Mask Message Direction"]
    #[inline(always)]
    #[must_use]
    pub fn can_if2msk2_mdir(&mut self) -> CAN_IF2MSK2_MDIR_W<IF2MSK2_SPEC, 14> {
        CAN_IF2MSK2_MDIR_W::new(self)
    }
    #[doc = "Bit 15 - Mask Extended Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn can_if2msk2_mxtd(&mut self) -> CAN_IF2MSK2_MXTD_W<IF2MSK2_SPEC, 15> {
        CAN_IF2MSK2_MXTD_W::new(self)
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
#[doc = "CAN IF2 Mask 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if2msk2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if2msk2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF2MSK2_SPEC;
impl crate::RegisterSpec for IF2MSK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if2msk2::R`](R) reader structure"]
impl crate::Readable for IF2MSK2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`if2msk2::W`](W) writer structure"]
impl crate::Writable for IF2MSK2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF2MSK2 to value 0"]
impl crate::Resettable for IF2MSK2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
