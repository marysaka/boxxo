#[doc = "Register `SCGC2` reader"]
pub type R = crate::R<SCGC2_SPEC>;
#[doc = "Register `SCGC2` writer"]
pub type W = crate::W<SCGC2_SPEC>;
#[doc = "Field `SYSCTL_SCGC2_GPIOA` reader - Port A Clock Gating Control"]
pub type SYSCTL_SCGC2_GPIOA_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGC2_GPIOA` writer - Port A Clock Gating Control"]
pub type SYSCTL_SCGC2_GPIOA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGC2_GPIOB` reader - Port B Clock Gating Control"]
pub type SYSCTL_SCGC2_GPIOB_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGC2_GPIOB` writer - Port B Clock Gating Control"]
pub type SYSCTL_SCGC2_GPIOB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGC2_GPIOC` reader - Port C Clock Gating Control"]
pub type SYSCTL_SCGC2_GPIOC_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGC2_GPIOC` writer - Port C Clock Gating Control"]
pub type SYSCTL_SCGC2_GPIOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGC2_GPIOD` reader - Port D Clock Gating Control"]
pub type SYSCTL_SCGC2_GPIOD_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGC2_GPIOD` writer - Port D Clock Gating Control"]
pub type SYSCTL_SCGC2_GPIOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGC2_GPIOE` reader - Port E Clock Gating Control"]
pub type SYSCTL_SCGC2_GPIOE_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGC2_GPIOE` writer - Port E Clock Gating Control"]
pub type SYSCTL_SCGC2_GPIOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGC2_GPIOF` reader - Port F Clock Gating Control"]
pub type SYSCTL_SCGC2_GPIOF_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGC2_GPIOF` writer - Port F Clock Gating Control"]
pub type SYSCTL_SCGC2_GPIOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGC2_UDMA` reader - Micro-DMA Clock Gating Control"]
pub type SYSCTL_SCGC2_UDMA_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGC2_UDMA` writer - Micro-DMA Clock Gating Control"]
pub type SYSCTL_SCGC2_UDMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGC2_USB0` reader - USB0 Clock Gating Control"]
pub type SYSCTL_SCGC2_USB0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGC2_USB0` writer - USB0 Clock Gating Control"]
pub type SYSCTL_SCGC2_USB0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Port A Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgc2_gpioa(&self) -> SYSCTL_SCGC2_GPIOA_R {
        SYSCTL_SCGC2_GPIOA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port B Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgc2_gpiob(&self) -> SYSCTL_SCGC2_GPIOB_R {
        SYSCTL_SCGC2_GPIOB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port C Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgc2_gpioc(&self) -> SYSCTL_SCGC2_GPIOC_R {
        SYSCTL_SCGC2_GPIOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port D Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgc2_gpiod(&self) -> SYSCTL_SCGC2_GPIOD_R {
        SYSCTL_SCGC2_GPIOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port E Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgc2_gpioe(&self) -> SYSCTL_SCGC2_GPIOE_R {
        SYSCTL_SCGC2_GPIOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port F Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgc2_gpiof(&self) -> SYSCTL_SCGC2_GPIOF_R {
        SYSCTL_SCGC2_GPIOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 13 - Micro-DMA Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgc2_udma(&self) -> SYSCTL_SCGC2_UDMA_R {
        SYSCTL_SCGC2_UDMA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - USB0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgc2_usb0(&self) -> SYSCTL_SCGC2_USB0_R {
        SYSCTL_SCGC2_USB0_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgc2_gpioa(&mut self) -> SYSCTL_SCGC2_GPIOA_W<SCGC2_SPEC, 0> {
        SYSCTL_SCGC2_GPIOA_W::new(self)
    }
    #[doc = "Bit 1 - Port B Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgc2_gpiob(&mut self) -> SYSCTL_SCGC2_GPIOB_W<SCGC2_SPEC, 1> {
        SYSCTL_SCGC2_GPIOB_W::new(self)
    }
    #[doc = "Bit 2 - Port C Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgc2_gpioc(&mut self) -> SYSCTL_SCGC2_GPIOC_W<SCGC2_SPEC, 2> {
        SYSCTL_SCGC2_GPIOC_W::new(self)
    }
    #[doc = "Bit 3 - Port D Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgc2_gpiod(&mut self) -> SYSCTL_SCGC2_GPIOD_W<SCGC2_SPEC, 3> {
        SYSCTL_SCGC2_GPIOD_W::new(self)
    }
    #[doc = "Bit 4 - Port E Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgc2_gpioe(&mut self) -> SYSCTL_SCGC2_GPIOE_W<SCGC2_SPEC, 4> {
        SYSCTL_SCGC2_GPIOE_W::new(self)
    }
    #[doc = "Bit 5 - Port F Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgc2_gpiof(&mut self) -> SYSCTL_SCGC2_GPIOF_W<SCGC2_SPEC, 5> {
        SYSCTL_SCGC2_GPIOF_W::new(self)
    }
    #[doc = "Bit 13 - Micro-DMA Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgc2_udma(&mut self) -> SYSCTL_SCGC2_UDMA_W<SCGC2_SPEC, 13> {
        SYSCTL_SCGC2_UDMA_W::new(self)
    }
    #[doc = "Bit 16 - USB0 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgc2_usb0(&mut self) -> SYSCTL_SCGC2_USB0_W<SCGC2_SPEC, 16> {
        SYSCTL_SCGC2_USB0_W::new(self)
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
#[doc = "Sleep Mode Clock Gating Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGC2_SPEC;
impl crate::RegisterSpec for SCGC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgc2::R`](R) reader structure"]
impl crate::Readable for SCGC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgc2::W`](W) writer structure"]
impl crate::Writable for SCGC2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGC2 to value 0"]
impl crate::Resettable for SCGC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
