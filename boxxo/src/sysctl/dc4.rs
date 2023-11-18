#[doc = "Register `DC4` reader"]
pub type R = crate::R<DC4_SPEC>;
#[doc = "Register `DC4` writer"]
pub type W = crate::W<DC4_SPEC>;
#[doc = "Field `SYSCTL_DC4_GPIOA` reader - GPIO Port A Present"]
pub type SYSCTL_DC4_GPIOA_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_GPIOA` writer - GPIO Port A Present"]
pub type SYSCTL_DC4_GPIOA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_GPIOB` reader - GPIO Port B Present"]
pub type SYSCTL_DC4_GPIOB_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_GPIOB` writer - GPIO Port B Present"]
pub type SYSCTL_DC4_GPIOB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_GPIOC` reader - GPIO Port C Present"]
pub type SYSCTL_DC4_GPIOC_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_GPIOC` writer - GPIO Port C Present"]
pub type SYSCTL_DC4_GPIOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_GPIOD` reader - GPIO Port D Present"]
pub type SYSCTL_DC4_GPIOD_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_GPIOD` writer - GPIO Port D Present"]
pub type SYSCTL_DC4_GPIOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_GPIOE` reader - GPIO Port E Present"]
pub type SYSCTL_DC4_GPIOE_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_GPIOE` writer - GPIO Port E Present"]
pub type SYSCTL_DC4_GPIOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_GPIOF` reader - GPIO Port F Present"]
pub type SYSCTL_DC4_GPIOF_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_GPIOF` writer - GPIO Port F Present"]
pub type SYSCTL_DC4_GPIOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_GPIOG` reader - GPIO Port G Present"]
pub type SYSCTL_DC4_GPIOG_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_GPIOG` writer - GPIO Port G Present"]
pub type SYSCTL_DC4_GPIOG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_GPIOH` reader - GPIO Port H Present"]
pub type SYSCTL_DC4_GPIOH_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_GPIOH` writer - GPIO Port H Present"]
pub type SYSCTL_DC4_GPIOH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_GPIOJ` reader - GPIO Port J Present"]
pub type SYSCTL_DC4_GPIOJ_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_GPIOJ` writer - GPIO Port J Present"]
pub type SYSCTL_DC4_GPIOJ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_ROM` reader - Internal Code ROM Present"]
pub type SYSCTL_DC4_ROM_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_ROM` writer - Internal Code ROM Present"]
pub type SYSCTL_DC4_ROM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_UDMA` reader - Micro-DMA Module Present"]
pub type SYSCTL_DC4_UDMA_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_UDMA` writer - Micro-DMA Module Present"]
pub type SYSCTL_DC4_UDMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_CCP6` reader - CCP6 Pin Present"]
pub type SYSCTL_DC4_CCP6_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_CCP6` writer - CCP6 Pin Present"]
pub type SYSCTL_DC4_CCP6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_CCP7` reader - CCP7 Pin Present"]
pub type SYSCTL_DC4_CCP7_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_CCP7` writer - CCP7 Pin Present"]
pub type SYSCTL_DC4_CCP7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_PICAL` reader - PIOSC Calibrate"]
pub type SYSCTL_DC4_PICAL_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_PICAL` writer - PIOSC Calibrate"]
pub type SYSCTL_DC4_PICAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_E1588` reader - 1588 Capable"]
pub type SYSCTL_DC4_E1588_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_E1588` writer - 1588 Capable"]
pub type SYSCTL_DC4_E1588_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_EMAC0` reader - Ethernet MAC Layer 0 Present"]
pub type SYSCTL_DC4_EMAC0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_EMAC0` writer - Ethernet MAC Layer 0 Present"]
pub type SYSCTL_DC4_EMAC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC4_EPHY0` reader - Ethernet PHY Layer 0 Present"]
pub type SYSCTL_DC4_EPHY0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC4_EPHY0` writer - Ethernet PHY Layer 0 Present"]
pub type SYSCTL_DC4_EPHY0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - GPIO Port A Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpioa(&self) -> SYSCTL_DC4_GPIOA_R {
        SYSCTL_DC4_GPIOA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpiob(&self) -> SYSCTL_DC4_GPIOB_R {
        SYSCTL_DC4_GPIOB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpioc(&self) -> SYSCTL_DC4_GPIOC_R {
        SYSCTL_DC4_GPIOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpiod(&self) -> SYSCTL_DC4_GPIOD_R {
        SYSCTL_DC4_GPIOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpioe(&self) -> SYSCTL_DC4_GPIOE_R {
        SYSCTL_DC4_GPIOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpiof(&self) -> SYSCTL_DC4_GPIOF_R {
        SYSCTL_DC4_GPIOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO Port G Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpiog(&self) -> SYSCTL_DC4_GPIOG_R {
        SYSCTL_DC4_GPIOG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO Port H Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpioh(&self) -> SYSCTL_DC4_GPIOH_R {
        SYSCTL_DC4_GPIOH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO Port J Present"]
    #[inline(always)]
    pub fn sysctl_dc4_gpioj(&self) -> SYSCTL_DC4_GPIOJ_R {
        SYSCTL_DC4_GPIOJ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Internal Code ROM Present"]
    #[inline(always)]
    pub fn sysctl_dc4_rom(&self) -> SYSCTL_DC4_ROM_R {
        SYSCTL_DC4_ROM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Micro-DMA Module Present"]
    #[inline(always)]
    pub fn sysctl_dc4_udma(&self) -> SYSCTL_DC4_UDMA_R {
        SYSCTL_DC4_UDMA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CCP6 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc4_ccp6(&self) -> SYSCTL_DC4_CCP6_R {
        SYSCTL_DC4_CCP6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CCP7 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc4_ccp7(&self) -> SYSCTL_DC4_CCP7_R {
        SYSCTL_DC4_CCP7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - PIOSC Calibrate"]
    #[inline(always)]
    pub fn sysctl_dc4_pical(&self) -> SYSCTL_DC4_PICAL_R {
        SYSCTL_DC4_PICAL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - 1588 Capable"]
    #[inline(always)]
    pub fn sysctl_dc4_e1588(&self) -> SYSCTL_DC4_E1588_R {
        SYSCTL_DC4_E1588_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Ethernet MAC Layer 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc4_emac0(&self) -> SYSCTL_DC4_EMAC0_R {
        SYSCTL_DC4_EMAC0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Ethernet PHY Layer 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc4_ephy0(&self) -> SYSCTL_DC4_EPHY0_R {
        SYSCTL_DC4_EPHY0_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO Port A Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_gpioa(&mut self) -> SYSCTL_DC4_GPIOA_W<DC4_SPEC, 0> {
        SYSCTL_DC4_GPIOA_W::new(self)
    }
    #[doc = "Bit 1 - GPIO Port B Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_gpiob(&mut self) -> SYSCTL_DC4_GPIOB_W<DC4_SPEC, 1> {
        SYSCTL_DC4_GPIOB_W::new(self)
    }
    #[doc = "Bit 2 - GPIO Port C Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_gpioc(&mut self) -> SYSCTL_DC4_GPIOC_W<DC4_SPEC, 2> {
        SYSCTL_DC4_GPIOC_W::new(self)
    }
    #[doc = "Bit 3 - GPIO Port D Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_gpiod(&mut self) -> SYSCTL_DC4_GPIOD_W<DC4_SPEC, 3> {
        SYSCTL_DC4_GPIOD_W::new(self)
    }
    #[doc = "Bit 4 - GPIO Port E Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_gpioe(&mut self) -> SYSCTL_DC4_GPIOE_W<DC4_SPEC, 4> {
        SYSCTL_DC4_GPIOE_W::new(self)
    }
    #[doc = "Bit 5 - GPIO Port F Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_gpiof(&mut self) -> SYSCTL_DC4_GPIOF_W<DC4_SPEC, 5> {
        SYSCTL_DC4_GPIOF_W::new(self)
    }
    #[doc = "Bit 6 - GPIO Port G Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_gpiog(&mut self) -> SYSCTL_DC4_GPIOG_W<DC4_SPEC, 6> {
        SYSCTL_DC4_GPIOG_W::new(self)
    }
    #[doc = "Bit 7 - GPIO Port H Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_gpioh(&mut self) -> SYSCTL_DC4_GPIOH_W<DC4_SPEC, 7> {
        SYSCTL_DC4_GPIOH_W::new(self)
    }
    #[doc = "Bit 8 - GPIO Port J Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_gpioj(&mut self) -> SYSCTL_DC4_GPIOJ_W<DC4_SPEC, 8> {
        SYSCTL_DC4_GPIOJ_W::new(self)
    }
    #[doc = "Bit 12 - Internal Code ROM Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_rom(&mut self) -> SYSCTL_DC4_ROM_W<DC4_SPEC, 12> {
        SYSCTL_DC4_ROM_W::new(self)
    }
    #[doc = "Bit 13 - Micro-DMA Module Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_udma(&mut self) -> SYSCTL_DC4_UDMA_W<DC4_SPEC, 13> {
        SYSCTL_DC4_UDMA_W::new(self)
    }
    #[doc = "Bit 14 - CCP6 Pin Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_ccp6(&mut self) -> SYSCTL_DC4_CCP6_W<DC4_SPEC, 14> {
        SYSCTL_DC4_CCP6_W::new(self)
    }
    #[doc = "Bit 15 - CCP7 Pin Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_ccp7(&mut self) -> SYSCTL_DC4_CCP7_W<DC4_SPEC, 15> {
        SYSCTL_DC4_CCP7_W::new(self)
    }
    #[doc = "Bit 18 - PIOSC Calibrate"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_pical(&mut self) -> SYSCTL_DC4_PICAL_W<DC4_SPEC, 18> {
        SYSCTL_DC4_PICAL_W::new(self)
    }
    #[doc = "Bit 24 - 1588 Capable"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_e1588(&mut self) -> SYSCTL_DC4_E1588_W<DC4_SPEC, 24> {
        SYSCTL_DC4_E1588_W::new(self)
    }
    #[doc = "Bit 28 - Ethernet MAC Layer 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_emac0(&mut self) -> SYSCTL_DC4_EMAC0_W<DC4_SPEC, 28> {
        SYSCTL_DC4_EMAC0_W::new(self)
    }
    #[doc = "Bit 30 - Ethernet PHY Layer 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc4_ephy0(&mut self) -> SYSCTL_DC4_EPHY0_W<DC4_SPEC, 30> {
        SYSCTL_DC4_EPHY0_W::new(self)
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
#[doc = "Device Capabilities 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC4_SPEC;
impl crate::RegisterSpec for DC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc4::R`](R) reader structure"]
impl crate::Readable for DC4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc4::W`](W) writer structure"]
impl crate::Writable for DC4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC4 to value 0"]
impl crate::Resettable for DC4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
