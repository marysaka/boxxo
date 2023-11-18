#[doc = "Register `PPGPIO` reader"]
pub type R = crate::R<PPGPIO_SPEC>;
#[doc = "Register `PPGPIO` writer"]
pub type W = crate::W<PPGPIO_SPEC>;
#[doc = "Field `SYSCTL_PPGPIO_P0` reader - GPIO Port A Present"]
pub type SYSCTL_PPGPIO_P0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPGPIO_P0` writer - GPIO Port A Present"]
pub type SYSCTL_PPGPIO_P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPGPIO_P1` reader - GPIO Port B Present"]
pub type SYSCTL_PPGPIO_P1_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPGPIO_P1` writer - GPIO Port B Present"]
pub type SYSCTL_PPGPIO_P1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPGPIO_P2` reader - GPIO Port C Present"]
pub type SYSCTL_PPGPIO_P2_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPGPIO_P2` writer - GPIO Port C Present"]
pub type SYSCTL_PPGPIO_P2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPGPIO_P3` reader - GPIO Port D Present"]
pub type SYSCTL_PPGPIO_P3_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPGPIO_P3` writer - GPIO Port D Present"]
pub type SYSCTL_PPGPIO_P3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPGPIO_P4` reader - GPIO Port E Present"]
pub type SYSCTL_PPGPIO_P4_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPGPIO_P4` writer - GPIO Port E Present"]
pub type SYSCTL_PPGPIO_P4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPGPIO_P5` reader - GPIO Port F Present"]
pub type SYSCTL_PPGPIO_P5_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPGPIO_P5` writer - GPIO Port F Present"]
pub type SYSCTL_PPGPIO_P5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPGPIO_P6` reader - GPIO Port G Present"]
pub type SYSCTL_PPGPIO_P6_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPGPIO_P6` writer - GPIO Port G Present"]
pub type SYSCTL_PPGPIO_P6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPGPIO_P7` reader - GPIO Port H Present"]
pub type SYSCTL_PPGPIO_P7_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPGPIO_P7` writer - GPIO Port H Present"]
pub type SYSCTL_PPGPIO_P7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPGPIO_P8` reader - GPIO Port J Present"]
pub type SYSCTL_PPGPIO_P8_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPGPIO_P8` writer - GPIO Port J Present"]
pub type SYSCTL_PPGPIO_P8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPGPIO_P9` reader - GPIO Port K Present"]
pub type SYSCTL_PPGPIO_P9_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPGPIO_P9` writer - GPIO Port K Present"]
pub type SYSCTL_PPGPIO_P9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPGPIO_P10` reader - GPIO Port L Present"]
pub type SYSCTL_PPGPIO_P10_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPGPIO_P10` writer - GPIO Port L Present"]
pub type SYSCTL_PPGPIO_P10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPGPIO_P11` reader - GPIO Port M Present"]
pub type SYSCTL_PPGPIO_P11_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPGPIO_P11` writer - GPIO Port M Present"]
pub type SYSCTL_PPGPIO_P11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPGPIO_P12` reader - GPIO Port N Present"]
pub type SYSCTL_PPGPIO_P12_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPGPIO_P12` writer - GPIO Port N Present"]
pub type SYSCTL_PPGPIO_P12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPGPIO_P13` reader - GPIO Port P Present"]
pub type SYSCTL_PPGPIO_P13_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPGPIO_P13` writer - GPIO Port P Present"]
pub type SYSCTL_PPGPIO_P13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPGPIO_P14` reader - GPIO Port Q Present"]
pub type SYSCTL_PPGPIO_P14_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPGPIO_P14` writer - GPIO Port Q Present"]
pub type SYSCTL_PPGPIO_P14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - GPIO Port A Present"]
    #[inline(always)]
    pub fn sysctl_ppgpio_p0(&self) -> SYSCTL_PPGPIO_P0_R {
        SYSCTL_PPGPIO_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Present"]
    #[inline(always)]
    pub fn sysctl_ppgpio_p1(&self) -> SYSCTL_PPGPIO_P1_R {
        SYSCTL_PPGPIO_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Present"]
    #[inline(always)]
    pub fn sysctl_ppgpio_p2(&self) -> SYSCTL_PPGPIO_P2_R {
        SYSCTL_PPGPIO_P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Present"]
    #[inline(always)]
    pub fn sysctl_ppgpio_p3(&self) -> SYSCTL_PPGPIO_P3_R {
        SYSCTL_PPGPIO_P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Present"]
    #[inline(always)]
    pub fn sysctl_ppgpio_p4(&self) -> SYSCTL_PPGPIO_P4_R {
        SYSCTL_PPGPIO_P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Present"]
    #[inline(always)]
    pub fn sysctl_ppgpio_p5(&self) -> SYSCTL_PPGPIO_P5_R {
        SYSCTL_PPGPIO_P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO Port G Present"]
    #[inline(always)]
    pub fn sysctl_ppgpio_p6(&self) -> SYSCTL_PPGPIO_P6_R {
        SYSCTL_PPGPIO_P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO Port H Present"]
    #[inline(always)]
    pub fn sysctl_ppgpio_p7(&self) -> SYSCTL_PPGPIO_P7_R {
        SYSCTL_PPGPIO_P7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO Port J Present"]
    #[inline(always)]
    pub fn sysctl_ppgpio_p8(&self) -> SYSCTL_PPGPIO_P8_R {
        SYSCTL_PPGPIO_P8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO Port K Present"]
    #[inline(always)]
    pub fn sysctl_ppgpio_p9(&self) -> SYSCTL_PPGPIO_P9_R {
        SYSCTL_PPGPIO_P9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO Port L Present"]
    #[inline(always)]
    pub fn sysctl_ppgpio_p10(&self) -> SYSCTL_PPGPIO_P10_R {
        SYSCTL_PPGPIO_P10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO Port M Present"]
    #[inline(always)]
    pub fn sysctl_ppgpio_p11(&self) -> SYSCTL_PPGPIO_P11_R {
        SYSCTL_PPGPIO_P11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO Port N Present"]
    #[inline(always)]
    pub fn sysctl_ppgpio_p12(&self) -> SYSCTL_PPGPIO_P12_R {
        SYSCTL_PPGPIO_P12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO Port P Present"]
    #[inline(always)]
    pub fn sysctl_ppgpio_p13(&self) -> SYSCTL_PPGPIO_P13_R {
        SYSCTL_PPGPIO_P13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO Port Q Present"]
    #[inline(always)]
    pub fn sysctl_ppgpio_p14(&self) -> SYSCTL_PPGPIO_P14_R {
        SYSCTL_PPGPIO_P14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO Port A Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppgpio_p0(&mut self) -> SYSCTL_PPGPIO_P0_W<PPGPIO_SPEC, 0> {
        SYSCTL_PPGPIO_P0_W::new(self)
    }
    #[doc = "Bit 1 - GPIO Port B Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppgpio_p1(&mut self) -> SYSCTL_PPGPIO_P1_W<PPGPIO_SPEC, 1> {
        SYSCTL_PPGPIO_P1_W::new(self)
    }
    #[doc = "Bit 2 - GPIO Port C Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppgpio_p2(&mut self) -> SYSCTL_PPGPIO_P2_W<PPGPIO_SPEC, 2> {
        SYSCTL_PPGPIO_P2_W::new(self)
    }
    #[doc = "Bit 3 - GPIO Port D Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppgpio_p3(&mut self) -> SYSCTL_PPGPIO_P3_W<PPGPIO_SPEC, 3> {
        SYSCTL_PPGPIO_P3_W::new(self)
    }
    #[doc = "Bit 4 - GPIO Port E Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppgpio_p4(&mut self) -> SYSCTL_PPGPIO_P4_W<PPGPIO_SPEC, 4> {
        SYSCTL_PPGPIO_P4_W::new(self)
    }
    #[doc = "Bit 5 - GPIO Port F Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppgpio_p5(&mut self) -> SYSCTL_PPGPIO_P5_W<PPGPIO_SPEC, 5> {
        SYSCTL_PPGPIO_P5_W::new(self)
    }
    #[doc = "Bit 6 - GPIO Port G Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppgpio_p6(&mut self) -> SYSCTL_PPGPIO_P6_W<PPGPIO_SPEC, 6> {
        SYSCTL_PPGPIO_P6_W::new(self)
    }
    #[doc = "Bit 7 - GPIO Port H Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppgpio_p7(&mut self) -> SYSCTL_PPGPIO_P7_W<PPGPIO_SPEC, 7> {
        SYSCTL_PPGPIO_P7_W::new(self)
    }
    #[doc = "Bit 8 - GPIO Port J Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppgpio_p8(&mut self) -> SYSCTL_PPGPIO_P8_W<PPGPIO_SPEC, 8> {
        SYSCTL_PPGPIO_P8_W::new(self)
    }
    #[doc = "Bit 9 - GPIO Port K Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppgpio_p9(&mut self) -> SYSCTL_PPGPIO_P9_W<PPGPIO_SPEC, 9> {
        SYSCTL_PPGPIO_P9_W::new(self)
    }
    #[doc = "Bit 10 - GPIO Port L Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppgpio_p10(&mut self) -> SYSCTL_PPGPIO_P10_W<PPGPIO_SPEC, 10> {
        SYSCTL_PPGPIO_P10_W::new(self)
    }
    #[doc = "Bit 11 - GPIO Port M Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppgpio_p11(&mut self) -> SYSCTL_PPGPIO_P11_W<PPGPIO_SPEC, 11> {
        SYSCTL_PPGPIO_P11_W::new(self)
    }
    #[doc = "Bit 12 - GPIO Port N Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppgpio_p12(&mut self) -> SYSCTL_PPGPIO_P12_W<PPGPIO_SPEC, 12> {
        SYSCTL_PPGPIO_P12_W::new(self)
    }
    #[doc = "Bit 13 - GPIO Port P Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppgpio_p13(&mut self) -> SYSCTL_PPGPIO_P13_W<PPGPIO_SPEC, 13> {
        SYSCTL_PPGPIO_P13_W::new(self)
    }
    #[doc = "Bit 14 - GPIO Port Q Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppgpio_p14(&mut self) -> SYSCTL_PPGPIO_P14_W<PPGPIO_SPEC, 14> {
        SYSCTL_PPGPIO_P14_W::new(self)
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
#[doc = "General-Purpose Input/Output Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppgpio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppgpio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPGPIO_SPEC;
impl crate::RegisterSpec for PPGPIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppgpio::R`](R) reader structure"]
impl crate::Readable for PPGPIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppgpio::W`](W) writer structure"]
impl crate::Writable for PPGPIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPGPIO to value 0"]
impl crate::Resettable for PPGPIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
