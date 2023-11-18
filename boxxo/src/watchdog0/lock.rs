#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LOCK_SPEC>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LOCK_SPEC>;
#[doc = "Field `WDT_LOCK` reader - Watchdog Lock"]
pub type WDT_LOCK_R = crate::FieldReader<WDT_LOCK_A>;
#[doc = "Watchdog Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum WDT_LOCK_A {
    #[doc = "0: Unlocked"]
    WDT_LOCK_UNLOCKED = 0,
    #[doc = "1: Locked"]
    WDT_LOCK_LOCKED = 1,
    #[doc = "449635665: Unlocks the watchdog timer"]
    WDT_LOCK_UNLOCK = 449635665,
}
impl From<WDT_LOCK_A> for u32 {
    #[inline(always)]
    fn from(variant: WDT_LOCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDT_LOCK_A {
    type Ux = u32;
}
impl WDT_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WDT_LOCK_A> {
        match self.bits {
            0 => Some(WDT_LOCK_A::WDT_LOCK_UNLOCKED),
            1 => Some(WDT_LOCK_A::WDT_LOCK_LOCKED),
            449635665 => Some(WDT_LOCK_A::WDT_LOCK_UNLOCK),
            _ => None,
        }
    }
    #[doc = "Unlocked"]
    #[inline(always)]
    pub fn is_wdt_lock_unlocked(&self) -> bool {
        *self == WDT_LOCK_A::WDT_LOCK_UNLOCKED
    }
    #[doc = "Locked"]
    #[inline(always)]
    pub fn is_wdt_lock_locked(&self) -> bool {
        *self == WDT_LOCK_A::WDT_LOCK_LOCKED
    }
    #[doc = "Unlocks the watchdog timer"]
    #[inline(always)]
    pub fn is_wdt_lock_unlock(&self) -> bool {
        *self == WDT_LOCK_A::WDT_LOCK_UNLOCK
    }
}
#[doc = "Field `WDT_LOCK` writer - Watchdog Lock"]
pub type WDT_LOCK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, WDT_LOCK_A>;
impl<'a, REG, const O: u8> WDT_LOCK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Unlocked"]
    #[inline(always)]
    pub fn wdt_lock_unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_LOCK_A::WDT_LOCK_UNLOCKED)
    }
    #[doc = "Locked"]
    #[inline(always)]
    pub fn wdt_lock_locked(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_LOCK_A::WDT_LOCK_LOCKED)
    }
    #[doc = "Unlocks the watchdog timer"]
    #[inline(always)]
    pub fn wdt_lock_unlock(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_LOCK_A::WDT_LOCK_UNLOCK)
    }
}
impl R {
    #[doc = "Bits 0:31 - Watchdog Lock"]
    #[inline(always)]
    pub fn wdt_lock(&self) -> WDT_LOCK_R {
        WDT_LOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_lock(&mut self) -> WDT_LOCK_W<LOCK_SPEC, 0> {
        WDT_LOCK_W::new(self)
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
#[doc = "Watchdog Lock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCK_SPEC;
impl crate::RegisterSpec for LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
