#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LOCK_SPEC>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LOCK_SPEC>;
#[doc = "Field `GPIO_LOCK` reader - GPIO Lock"]
pub type GPIO_LOCK_R = crate::FieldReader<GPIO_LOCK_A>;
#[doc = "GPIO Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GPIO_LOCK_A {
    #[doc = "0: The GPIOCR register is unlocked and may be modified"]
    GPIO_LOCK_UNLOCKED = 0,
    #[doc = "1: The GPIOCR register is locked and may not be modified"]
    GPIO_LOCK_LOCKED = 1,
    #[doc = "1280262987: Unlocks the GPIO_CR register"]
    GPIO_LOCK_KEY = 1280262987,
}
impl From<GPIO_LOCK_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_LOCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPIO_LOCK_A {
    type Ux = u32;
}
impl GPIO_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GPIO_LOCK_A> {
        match self.bits {
            0 => Some(GPIO_LOCK_A::GPIO_LOCK_UNLOCKED),
            1 => Some(GPIO_LOCK_A::GPIO_LOCK_LOCKED),
            1280262987 => Some(GPIO_LOCK_A::GPIO_LOCK_KEY),
            _ => None,
        }
    }
    #[doc = "The GPIOCR register is unlocked and may be modified"]
    #[inline(always)]
    pub fn is_gpio_lock_unlocked(&self) -> bool {
        *self == GPIO_LOCK_A::GPIO_LOCK_UNLOCKED
    }
    #[doc = "The GPIOCR register is locked and may not be modified"]
    #[inline(always)]
    pub fn is_gpio_lock_locked(&self) -> bool {
        *self == GPIO_LOCK_A::GPIO_LOCK_LOCKED
    }
    #[doc = "Unlocks the GPIO_CR register"]
    #[inline(always)]
    pub fn is_gpio_lock_key(&self) -> bool {
        *self == GPIO_LOCK_A::GPIO_LOCK_KEY
    }
}
#[doc = "Field `GPIO_LOCK` writer - GPIO Lock"]
pub type GPIO_LOCK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, GPIO_LOCK_A>;
impl<'a, REG, const O: u8> GPIO_LOCK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "The GPIOCR register is unlocked and may be modified"]
    #[inline(always)]
    pub fn gpio_lock_unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_LOCK_A::GPIO_LOCK_UNLOCKED)
    }
    #[doc = "The GPIOCR register is locked and may not be modified"]
    #[inline(always)]
    pub fn gpio_lock_locked(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_LOCK_A::GPIO_LOCK_LOCKED)
    }
    #[doc = "Unlocks the GPIO_CR register"]
    #[inline(always)]
    pub fn gpio_lock_key(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO_LOCK_A::GPIO_LOCK_KEY)
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO Lock"]
    #[inline(always)]
    pub fn gpio_lock(&self) -> GPIO_LOCK_R {
        GPIO_LOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO Lock"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_lock(&mut self) -> GPIO_LOCK_W<LOCK_SPEC, 0> {
        GPIO_LOCK_W::new(self)
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
#[doc = "GPIO Lock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
