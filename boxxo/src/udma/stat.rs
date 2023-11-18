#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `UDMA_STAT_MASTEN` reader - Master Enable Status"]
pub type UDMA_STAT_MASTEN_R = crate::BitReader;
#[doc = "Field `UDMA_STAT_MASTEN` writer - Master Enable Status"]
pub type UDMA_STAT_MASTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UDMA_STAT_STATE` reader - Control State Machine Status"]
pub type UDMA_STAT_STATE_R = crate::FieldReader<UDMA_STAT_STATE_A>;
#[doc = "Control State Machine Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UDMA_STAT_STATE_A {
    #[doc = "0: Idle"]
    UDMA_STAT_STATE_IDLE = 0,
    #[doc = "1: Reading channel controller data"]
    UDMA_STAT_STATE_RD_CTRL = 1,
    #[doc = "2: Reading source end pointer"]
    UDMA_STAT_STATE_RD_SRCENDP = 2,
    #[doc = "3: Reading destination end pointer"]
    UDMA_STAT_STATE_RD_DSTENDP = 3,
    #[doc = "4: Reading source data"]
    UDMA_STAT_STATE_RD_SRCDAT = 4,
    #[doc = "5: Writing destination data"]
    UDMA_STAT_STATE_WR_DSTDAT = 5,
    #[doc = "6: Waiting for uDMA request to clear"]
    UDMA_STAT_STATE_WAIT = 6,
    #[doc = "7: Writing channel controller data"]
    UDMA_STAT_STATE_WR_CTRL = 7,
    #[doc = "8: Stalled"]
    UDMA_STAT_STATE_STALL = 8,
    #[doc = "9: Done"]
    UDMA_STAT_STATE_DONE = 9,
    #[doc = "10: Undefined"]
    UDMA_STAT_STATE_UNDEF = 10,
}
impl From<UDMA_STAT_STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: UDMA_STAT_STATE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UDMA_STAT_STATE_A {
    type Ux = u8;
}
impl UDMA_STAT_STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UDMA_STAT_STATE_A> {
        match self.bits {
            0 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_IDLE),
            1 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_CTRL),
            2 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_SRCENDP),
            3 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_DSTENDP),
            4 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_SRCDAT),
            5 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_WR_DSTDAT),
            6 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_WAIT),
            7 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_WR_CTRL),
            8 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_STALL),
            9 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_DONE),
            10 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_UNDEF),
            _ => None,
        }
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_udma_stat_state_idle(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_IDLE
    }
    #[doc = "Reading channel controller data"]
    #[inline(always)]
    pub fn is_udma_stat_state_rd_ctrl(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_CTRL
    }
    #[doc = "Reading source end pointer"]
    #[inline(always)]
    pub fn is_udma_stat_state_rd_srcendp(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_SRCENDP
    }
    #[doc = "Reading destination end pointer"]
    #[inline(always)]
    pub fn is_udma_stat_state_rd_dstendp(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_DSTENDP
    }
    #[doc = "Reading source data"]
    #[inline(always)]
    pub fn is_udma_stat_state_rd_srcdat(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_SRCDAT
    }
    #[doc = "Writing destination data"]
    #[inline(always)]
    pub fn is_udma_stat_state_wr_dstdat(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_WR_DSTDAT
    }
    #[doc = "Waiting for uDMA request to clear"]
    #[inline(always)]
    pub fn is_udma_stat_state_wait(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_WAIT
    }
    #[doc = "Writing channel controller data"]
    #[inline(always)]
    pub fn is_udma_stat_state_wr_ctrl(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_WR_CTRL
    }
    #[doc = "Stalled"]
    #[inline(always)]
    pub fn is_udma_stat_state_stall(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_STALL
    }
    #[doc = "Done"]
    #[inline(always)]
    pub fn is_udma_stat_state_done(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_DONE
    }
    #[doc = "Undefined"]
    #[inline(always)]
    pub fn is_udma_stat_state_undef(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_UNDEF
    }
}
#[doc = "Field `UDMA_STAT_STATE` writer - Control State Machine Status"]
pub type UDMA_STAT_STATE_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 4, O, UDMA_STAT_STATE_A>;
impl<'a, REG, const O: u8> UDMA_STAT_STATE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Idle"]
    #[inline(always)]
    pub fn udma_stat_state_idle(self) -> &'a mut crate::W<REG> {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_IDLE)
    }
    #[doc = "Reading channel controller data"]
    #[inline(always)]
    pub fn udma_stat_state_rd_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_CTRL)
    }
    #[doc = "Reading source end pointer"]
    #[inline(always)]
    pub fn udma_stat_state_rd_srcendp(self) -> &'a mut crate::W<REG> {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_SRCENDP)
    }
    #[doc = "Reading destination end pointer"]
    #[inline(always)]
    pub fn udma_stat_state_rd_dstendp(self) -> &'a mut crate::W<REG> {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_DSTENDP)
    }
    #[doc = "Reading source data"]
    #[inline(always)]
    pub fn udma_stat_state_rd_srcdat(self) -> &'a mut crate::W<REG> {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_SRCDAT)
    }
    #[doc = "Writing destination data"]
    #[inline(always)]
    pub fn udma_stat_state_wr_dstdat(self) -> &'a mut crate::W<REG> {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_WR_DSTDAT)
    }
    #[doc = "Waiting for uDMA request to clear"]
    #[inline(always)]
    pub fn udma_stat_state_wait(self) -> &'a mut crate::W<REG> {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_WAIT)
    }
    #[doc = "Writing channel controller data"]
    #[inline(always)]
    pub fn udma_stat_state_wr_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_WR_CTRL)
    }
    #[doc = "Stalled"]
    #[inline(always)]
    pub fn udma_stat_state_stall(self) -> &'a mut crate::W<REG> {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_STALL)
    }
    #[doc = "Done"]
    #[inline(always)]
    pub fn udma_stat_state_done(self) -> &'a mut crate::W<REG> {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_DONE)
    }
    #[doc = "Undefined"]
    #[inline(always)]
    pub fn udma_stat_state_undef(self) -> &'a mut crate::W<REG> {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_UNDEF)
    }
}
#[doc = "Field `UDMA_STAT_DMACHANS` reader - Available uDMA Channels Minus 1"]
pub type UDMA_STAT_DMACHANS_R = crate::FieldReader;
#[doc = "Field `UDMA_STAT_DMACHANS` writer - Available uDMA Channels Minus 1"]
pub type UDMA_STAT_DMACHANS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bit 0 - Master Enable Status"]
    #[inline(always)]
    pub fn udma_stat_masten(&self) -> UDMA_STAT_MASTEN_R {
        UDMA_STAT_MASTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Control State Machine Status"]
    #[inline(always)]
    pub fn udma_stat_state(&self) -> UDMA_STAT_STATE_R {
        UDMA_STAT_STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Available uDMA Channels Minus 1"]
    #[inline(always)]
    pub fn udma_stat_dmachans(&self) -> UDMA_STAT_DMACHANS_R {
        UDMA_STAT_DMACHANS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master Enable Status"]
    #[inline(always)]
    #[must_use]
    pub fn udma_stat_masten(&mut self) -> UDMA_STAT_MASTEN_W<STAT_SPEC, 0> {
        UDMA_STAT_MASTEN_W::new(self)
    }
    #[doc = "Bits 4:7 - Control State Machine Status"]
    #[inline(always)]
    #[must_use]
    pub fn udma_stat_state(&mut self) -> UDMA_STAT_STATE_W<STAT_SPEC, 4> {
        UDMA_STAT_STATE_W::new(self)
    }
    #[doc = "Bits 16:20 - Available uDMA Channels Minus 1"]
    #[inline(always)]
    #[must_use]
    pub fn udma_stat_dmachans(&mut self) -> UDMA_STAT_DMACHANS_W<STAT_SPEC, 16> {
        UDMA_STAT_DMACHANS_W::new(self)
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
#[doc = "DMA Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
