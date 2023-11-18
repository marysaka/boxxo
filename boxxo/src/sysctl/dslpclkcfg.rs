#[doc = "Register `DSLPCLKCFG` reader"]
pub type R = crate::R<DSLPCLKCFG_SPEC>;
#[doc = "Register `DSLPCLKCFG` writer"]
pub type W = crate::W<DSLPCLKCFG_SPEC>;
#[doc = "Field `SYSCTL_DSLPCLKCFG_O` reader - Clock Source"]
pub type SYSCTL_DSLPCLKCFG_O_R = crate::FieldReader<SYSCTL_DSLPCLKCFG_O_A>;
#[doc = "Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_DSLPCLKCFG_O_A {
    #[doc = "0: MOSC"]
    SYSCTL_DSLPCLKCFG_O_IGN = 0,
    #[doc = "1: PIOSC"]
    SYSCTL_DSLPCLKCFG_O_IO = 1,
    #[doc = "3: 30 kHz"]
    SYSCTL_DSLPCLKCFG_O_30 = 3,
    #[doc = "7: 32.768 kHz"]
    SYSCTL_DSLPCLKCFG_O_32 = 7,
}
impl From<SYSCTL_DSLPCLKCFG_O_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DSLPCLKCFG_O_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_DSLPCLKCFG_O_A {
    type Ux = u8;
}
impl SYSCTL_DSLPCLKCFG_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_DSLPCLKCFG_O_A> {
        match self.bits {
            0 => Some(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_IGN),
            1 => Some(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_IO),
            3 => Some(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_30),
            7 => Some(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_32),
            _ => None,
        }
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_o_ign(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_IGN
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_o_io(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_IO
    }
    #[doc = "30 kHz"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_o_30(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_30
    }
    #[doc = "32.768 kHz"]
    #[inline(always)]
    pub fn is_sysctl_dslpclkcfg_o_32(&self) -> bool {
        *self == SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_32
    }
}
#[doc = "Field `SYSCTL_DSLPCLKCFG_O` writer - Clock Source"]
pub type SYSCTL_DSLPCLKCFG_O_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O, SYSCTL_DSLPCLKCFG_O_A>;
impl<'a, REG, const O: u8> SYSCTL_DSLPCLKCFG_O_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_o_ign(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_IGN)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_o_io(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_IO)
    }
    #[doc = "30 kHz"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_o_30(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_30)
    }
    #[doc = "32.768 kHz"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_o_32(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DSLPCLKCFG_O_A::SYSCTL_DSLPCLKCFG_O_32)
    }
}
#[doc = "Field `SYSCTL_DSLPCLKCFG_D` reader - Divider Field Override"]
pub type SYSCTL_DSLPCLKCFG_D_R = crate::FieldReader;
#[doc = "Field `SYSCTL_DSLPCLKCFG_D` writer - Divider Field Override"]
pub type SYSCTL_DSLPCLKCFG_D_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 4:6 - Clock Source"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_o(&self) -> SYSCTL_DSLPCLKCFG_O_R {
        SYSCTL_DSLPCLKCFG_O_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 23:28 - Divider Field Override"]
    #[inline(always)]
    pub fn sysctl_dslpclkcfg_d(&self) -> SYSCTL_DSLPCLKCFG_D_R {
        SYSCTL_DSLPCLKCFG_D_R::new(((self.bits >> 23) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dslpclkcfg_o(&mut self) -> SYSCTL_DSLPCLKCFG_O_W<DSLPCLKCFG_SPEC, 4> {
        SYSCTL_DSLPCLKCFG_O_W::new(self)
    }
    #[doc = "Bits 23:28 - Divider Field Override"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dslpclkcfg_d(&mut self) -> SYSCTL_DSLPCLKCFG_D_W<DSLPCLKCFG_SPEC, 23> {
        SYSCTL_DSLPCLKCFG_D_W::new(self)
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
#[doc = "Deep Sleep Clock Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dslpclkcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dslpclkcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSLPCLKCFG_SPEC;
impl crate::RegisterSpec for DSLPCLKCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dslpclkcfg::R`](R) reader structure"]
impl crate::Readable for DSLPCLKCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dslpclkcfg::W`](W) writer structure"]
impl crate::Writable for DSLPCLKCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSLPCLKCFG to value 0"]
impl crate::Resettable for DSLPCLKCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
