#[doc = "Register `GPIOHBCTL` reader"]
pub type R = crate::R<GPIOHBCTL_SPEC>;
#[doc = "Register `GPIOHBCTL` writer"]
pub type W = crate::W<GPIOHBCTL_SPEC>;
#[doc = "Field `SYSCTL_GPIOHBCTL_PORTA` reader - Port A Advanced High-Performance Bus"]
pub type SYSCTL_GPIOHBCTL_PORTA_R = crate::BitReader;
#[doc = "Field `SYSCTL_GPIOHBCTL_PORTA` writer - Port A Advanced High-Performance Bus"]
pub type SYSCTL_GPIOHBCTL_PORTA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_GPIOHBCTL_PORTB` reader - Port B Advanced High-Performance Bus"]
pub type SYSCTL_GPIOHBCTL_PORTB_R = crate::BitReader;
#[doc = "Field `SYSCTL_GPIOHBCTL_PORTB` writer - Port B Advanced High-Performance Bus"]
pub type SYSCTL_GPIOHBCTL_PORTB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_GPIOHBCTL_PORTC` reader - Port C Advanced High-Performance Bus"]
pub type SYSCTL_GPIOHBCTL_PORTC_R = crate::BitReader;
#[doc = "Field `SYSCTL_GPIOHBCTL_PORTC` writer - Port C Advanced High-Performance Bus"]
pub type SYSCTL_GPIOHBCTL_PORTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_GPIOHBCTL_PORTD` reader - Port D Advanced High-Performance Bus"]
pub type SYSCTL_GPIOHBCTL_PORTD_R = crate::BitReader;
#[doc = "Field `SYSCTL_GPIOHBCTL_PORTD` writer - Port D Advanced High-Performance Bus"]
pub type SYSCTL_GPIOHBCTL_PORTD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_GPIOHBCTL_PORTE` reader - Port E Advanced High-Performance Bus"]
pub type SYSCTL_GPIOHBCTL_PORTE_R = crate::BitReader;
#[doc = "Field `SYSCTL_GPIOHBCTL_PORTE` writer - Port E Advanced High-Performance Bus"]
pub type SYSCTL_GPIOHBCTL_PORTE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_GPIOHBCTL_PORTF` reader - Port F Advanced High-Performance Bus"]
pub type SYSCTL_GPIOHBCTL_PORTF_R = crate::BitReader;
#[doc = "Field `SYSCTL_GPIOHBCTL_PORTF` writer - Port F Advanced High-Performance Bus"]
pub type SYSCTL_GPIOHBCTL_PORTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Port A Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn sysctl_gpiohbctl_porta(&self) -> SYSCTL_GPIOHBCTL_PORTA_R {
        SYSCTL_GPIOHBCTL_PORTA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port B Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn sysctl_gpiohbctl_portb(&self) -> SYSCTL_GPIOHBCTL_PORTB_R {
        SYSCTL_GPIOHBCTL_PORTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port C Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn sysctl_gpiohbctl_portc(&self) -> SYSCTL_GPIOHBCTL_PORTC_R {
        SYSCTL_GPIOHBCTL_PORTC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port D Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn sysctl_gpiohbctl_portd(&self) -> SYSCTL_GPIOHBCTL_PORTD_R {
        SYSCTL_GPIOHBCTL_PORTD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port E Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn sysctl_gpiohbctl_porte(&self) -> SYSCTL_GPIOHBCTL_PORTE_R {
        SYSCTL_GPIOHBCTL_PORTE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port F Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn sysctl_gpiohbctl_portf(&self) -> SYSCTL_GPIOHBCTL_PORTF_R {
        SYSCTL_GPIOHBCTL_PORTF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A Advanced High-Performance Bus"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_gpiohbctl_porta(&mut self) -> SYSCTL_GPIOHBCTL_PORTA_W<GPIOHBCTL_SPEC, 0> {
        SYSCTL_GPIOHBCTL_PORTA_W::new(self)
    }
    #[doc = "Bit 1 - Port B Advanced High-Performance Bus"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_gpiohbctl_portb(&mut self) -> SYSCTL_GPIOHBCTL_PORTB_W<GPIOHBCTL_SPEC, 1> {
        SYSCTL_GPIOHBCTL_PORTB_W::new(self)
    }
    #[doc = "Bit 2 - Port C Advanced High-Performance Bus"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_gpiohbctl_portc(&mut self) -> SYSCTL_GPIOHBCTL_PORTC_W<GPIOHBCTL_SPEC, 2> {
        SYSCTL_GPIOHBCTL_PORTC_W::new(self)
    }
    #[doc = "Bit 3 - Port D Advanced High-Performance Bus"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_gpiohbctl_portd(&mut self) -> SYSCTL_GPIOHBCTL_PORTD_W<GPIOHBCTL_SPEC, 3> {
        SYSCTL_GPIOHBCTL_PORTD_W::new(self)
    }
    #[doc = "Bit 4 - Port E Advanced High-Performance Bus"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_gpiohbctl_porte(&mut self) -> SYSCTL_GPIOHBCTL_PORTE_W<GPIOHBCTL_SPEC, 4> {
        SYSCTL_GPIOHBCTL_PORTE_W::new(self)
    }
    #[doc = "Bit 5 - Port F Advanced High-Performance Bus"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_gpiohbctl_portf(&mut self) -> SYSCTL_GPIOHBCTL_PORTF_W<GPIOHBCTL_SPEC, 5> {
        SYSCTL_GPIOHBCTL_PORTF_W::new(self)
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
#[doc = "GPIO High-Performance Bus Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiohbctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiohbctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOHBCTL_SPEC;
impl crate::RegisterSpec for GPIOHBCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiohbctl::R`](R) reader structure"]
impl crate::Readable for GPIOHBCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpiohbctl::W`](W) writer structure"]
impl crate::Writable for GPIOHBCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOHBCTL to value 0"]
impl crate::Resettable for GPIOHBCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
