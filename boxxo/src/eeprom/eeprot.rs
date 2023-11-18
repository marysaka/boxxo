#[doc = "Register `EEPROT` reader"]
pub type R = crate::R<EEPROT_SPEC>;
#[doc = "Register `EEPROT` writer"]
pub type W = crate::W<EEPROT_SPEC>;
#[doc = "Field `EEPROM_EEPROT_PROT` reader - Protection Control"]
pub type EEPROM_EEPROT_PROT_R = crate::FieldReader<EEPROM_EEPROT_PROT_A>;
#[doc = "Protection Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EEPROM_EEPROT_PROT_A {
    #[doc = "0: This setting is the default. If there is no password, the block is not protected and is readable and writable"]
    EEPROM_EEPROT_PROT_RWNPW = 0,
    #[doc = "1: If there is a password, the block is readable or writable only when unlocked"]
    EEPROM_EEPROT_PROT_RWPW = 1,
    #[doc = "2: If there is no password, the block is readable, not writable"]
    EEPROM_EEPROT_PROT_RONPW = 2,
}
impl From<EEPROM_EEPROT_PROT_A> for u8 {
    #[inline(always)]
    fn from(variant: EEPROM_EEPROT_PROT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EEPROM_EEPROT_PROT_A {
    type Ux = u8;
}
impl EEPROM_EEPROT_PROT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EEPROM_EEPROT_PROT_A> {
        match self.bits {
            0 => Some(EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RWNPW),
            1 => Some(EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RWPW),
            2 => Some(EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RONPW),
            _ => None,
        }
    }
    #[doc = "This setting is the default. If there is no password, the block is not protected and is readable and writable"]
    #[inline(always)]
    pub fn is_eeprom_eeprot_prot_rwnpw(&self) -> bool {
        *self == EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RWNPW
    }
    #[doc = "If there is a password, the block is readable or writable only when unlocked"]
    #[inline(always)]
    pub fn is_eeprom_eeprot_prot_rwpw(&self) -> bool {
        *self == EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RWPW
    }
    #[doc = "If there is no password, the block is readable, not writable"]
    #[inline(always)]
    pub fn is_eeprom_eeprot_prot_ronpw(&self) -> bool {
        *self == EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RONPW
    }
}
#[doc = "Field `EEPROM_EEPROT_PROT` writer - Protection Control"]
pub type EEPROM_EEPROT_PROT_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O, EEPROM_EEPROT_PROT_A>;
impl<'a, REG, const O: u8> EEPROM_EEPROT_PROT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This setting is the default. If there is no password, the block is not protected and is readable and writable"]
    #[inline(always)]
    pub fn eeprom_eeprot_prot_rwnpw(self) -> &'a mut crate::W<REG> {
        self.variant(EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RWNPW)
    }
    #[doc = "If there is a password, the block is readable or writable only when unlocked"]
    #[inline(always)]
    pub fn eeprom_eeprot_prot_rwpw(self) -> &'a mut crate::W<REG> {
        self.variant(EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RWPW)
    }
    #[doc = "If there is no password, the block is readable, not writable"]
    #[inline(always)]
    pub fn eeprom_eeprot_prot_ronpw(self) -> &'a mut crate::W<REG> {
        self.variant(EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RONPW)
    }
}
#[doc = "Field `EEPROM_EEPROT_ACC` reader - Access Control"]
pub type EEPROM_EEPROT_ACC_R = crate::BitReader;
#[doc = "Field `EEPROM_EEPROT_ACC` writer - Access Control"]
pub type EEPROM_EEPROT_ACC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Protection Control"]
    #[inline(always)]
    pub fn eeprom_eeprot_prot(&self) -> EEPROM_EEPROT_PROT_R {
        EEPROM_EEPROT_PROT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Access Control"]
    #[inline(always)]
    pub fn eeprom_eeprot_acc(&self) -> EEPROM_EEPROT_ACC_R {
        EEPROM_EEPROT_ACC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Protection Control"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eeprot_prot(&mut self) -> EEPROM_EEPROT_PROT_W<EEPROT_SPEC, 0> {
        EEPROM_EEPROT_PROT_W::new(self)
    }
    #[doc = "Bit 3 - Access Control"]
    #[inline(always)]
    #[must_use]
    pub fn eeprom_eeprot_acc(&mut self) -> EEPROM_EEPROT_ACC_W<EEPROT_SPEC, 3> {
        EEPROM_EEPROT_ACC_W::new(self)
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
#[doc = "EEPROM Protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeprot::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeprot::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEPROT_SPEC;
impl crate::RegisterSpec for EEPROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eeprot::R`](R) reader structure"]
impl crate::Readable for EEPROT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eeprot::W`](W) writer structure"]
impl crate::Writable for EEPROT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEPROT to value 0"]
impl crate::Resettable for EEPROT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
