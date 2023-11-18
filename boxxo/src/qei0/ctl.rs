#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `QEI_CTL_ENABLE` reader - Enable QEI"]
pub type QEI_CTL_ENABLE_R = crate::BitReader;
#[doc = "Field `QEI_CTL_ENABLE` writer - Enable QEI"]
pub type QEI_CTL_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_CTL_SWAP` reader - Swap Signals"]
pub type QEI_CTL_SWAP_R = crate::BitReader;
#[doc = "Field `QEI_CTL_SWAP` writer - Swap Signals"]
pub type QEI_CTL_SWAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_CTL_SIGMODE` reader - Signal Mode"]
pub type QEI_CTL_SIGMODE_R = crate::BitReader;
#[doc = "Field `QEI_CTL_SIGMODE` writer - Signal Mode"]
pub type QEI_CTL_SIGMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_CTL_CAPMODE` reader - Capture Mode"]
pub type QEI_CTL_CAPMODE_R = crate::BitReader;
#[doc = "Field `QEI_CTL_CAPMODE` writer - Capture Mode"]
pub type QEI_CTL_CAPMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_CTL_RESMODE` reader - Reset Mode"]
pub type QEI_CTL_RESMODE_R = crate::BitReader;
#[doc = "Field `QEI_CTL_RESMODE` writer - Reset Mode"]
pub type QEI_CTL_RESMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_CTL_VELEN` reader - Capture Velocity"]
pub type QEI_CTL_VELEN_R = crate::BitReader;
#[doc = "Field `QEI_CTL_VELEN` writer - Capture Velocity"]
pub type QEI_CTL_VELEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_CTL_VELDIV` reader - Predivide Velocity"]
pub type QEI_CTL_VELDIV_R = crate::FieldReader<QEI_CTL_VELDIV_A>;
#[doc = "Predivide Velocity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QEI_CTL_VELDIV_A {
    #[doc = "0: QEI clock /1"]
    QEI_CTL_VELDIV_1 = 0,
    #[doc = "1: QEI clock /2"]
    QEI_CTL_VELDIV_2 = 1,
    #[doc = "2: QEI clock /4"]
    QEI_CTL_VELDIV_4 = 2,
    #[doc = "3: QEI clock /8"]
    QEI_CTL_VELDIV_8 = 3,
    #[doc = "4: QEI clock /16"]
    QEI_CTL_VELDIV_16 = 4,
    #[doc = "5: QEI clock /32"]
    QEI_CTL_VELDIV_32 = 5,
    #[doc = "6: QEI clock /64"]
    QEI_CTL_VELDIV_64 = 6,
    #[doc = "7: QEI clock /128"]
    QEI_CTL_VELDIV_128 = 7,
}
impl From<QEI_CTL_VELDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: QEI_CTL_VELDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QEI_CTL_VELDIV_A {
    type Ux = u8;
}
impl QEI_CTL_VELDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> QEI_CTL_VELDIV_A {
        match self.bits {
            0 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_1,
            1 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_2,
            2 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_4,
            3 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_8,
            4 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_16,
            5 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_32,
            6 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_64,
            7 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_128,
            _ => unreachable!(),
        }
    }
    #[doc = "QEI clock /1"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_1(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_1
    }
    #[doc = "QEI clock /2"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_2(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_2
    }
    #[doc = "QEI clock /4"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_4(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_4
    }
    #[doc = "QEI clock /8"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_8(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_8
    }
    #[doc = "QEI clock /16"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_16(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_16
    }
    #[doc = "QEI clock /32"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_32(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_32
    }
    #[doc = "QEI clock /64"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_64(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_64
    }
    #[doc = "QEI clock /128"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_128(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_128
    }
}
#[doc = "Field `QEI_CTL_VELDIV` writer - Predivide Velocity"]
pub type QEI_CTL_VELDIV_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 3, O, QEI_CTL_VELDIV_A>;
impl<'a, REG, const O: u8> QEI_CTL_VELDIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "QEI clock /1"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_1(self) -> &'a mut crate::W<REG> {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_1)
    }
    #[doc = "QEI clock /2"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_2(self) -> &'a mut crate::W<REG> {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_2)
    }
    #[doc = "QEI clock /4"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_4(self) -> &'a mut crate::W<REG> {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_4)
    }
    #[doc = "QEI clock /8"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_8(self) -> &'a mut crate::W<REG> {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_8)
    }
    #[doc = "QEI clock /16"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_16(self) -> &'a mut crate::W<REG> {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_16)
    }
    #[doc = "QEI clock /32"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_32(self) -> &'a mut crate::W<REG> {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_32)
    }
    #[doc = "QEI clock /64"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_64(self) -> &'a mut crate::W<REG> {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_64)
    }
    #[doc = "QEI clock /128"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_128(self) -> &'a mut crate::W<REG> {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_128)
    }
}
#[doc = "Field `QEI_CTL_INVA` reader - Invert PhA"]
pub type QEI_CTL_INVA_R = crate::BitReader;
#[doc = "Field `QEI_CTL_INVA` writer - Invert PhA"]
pub type QEI_CTL_INVA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_CTL_INVB` reader - Invert PhB"]
pub type QEI_CTL_INVB_R = crate::BitReader;
#[doc = "Field `QEI_CTL_INVB` writer - Invert PhB"]
pub type QEI_CTL_INVB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_CTL_INVI` reader - Invert Index Pulse"]
pub type QEI_CTL_INVI_R = crate::BitReader;
#[doc = "Field `QEI_CTL_INVI` writer - Invert Index Pulse"]
pub type QEI_CTL_INVI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_CTL_STALLEN` reader - Stall QEI"]
pub type QEI_CTL_STALLEN_R = crate::BitReader;
#[doc = "Field `QEI_CTL_STALLEN` writer - Stall QEI"]
pub type QEI_CTL_STALLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_CTL_FILTEN` reader - Enable Input Filter"]
pub type QEI_CTL_FILTEN_R = crate::BitReader;
#[doc = "Field `QEI_CTL_FILTEN` writer - Enable Input Filter"]
pub type QEI_CTL_FILTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_CTL_FILTCNT` reader - Input Filter Prescale Count"]
pub type QEI_CTL_FILTCNT_R = crate::FieldReader;
#[doc = "Field `QEI_CTL_FILTCNT` writer - Input Filter Prescale Count"]
pub type QEI_CTL_FILTCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Enable QEI"]
    #[inline(always)]
    pub fn qei_ctl_enable(&self) -> QEI_CTL_ENABLE_R {
        QEI_CTL_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Swap Signals"]
    #[inline(always)]
    pub fn qei_ctl_swap(&self) -> QEI_CTL_SWAP_R {
        QEI_CTL_SWAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Signal Mode"]
    #[inline(always)]
    pub fn qei_ctl_sigmode(&self) -> QEI_CTL_SIGMODE_R {
        QEI_CTL_SIGMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture Mode"]
    #[inline(always)]
    pub fn qei_ctl_capmode(&self) -> QEI_CTL_CAPMODE_R {
        QEI_CTL_CAPMODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset Mode"]
    #[inline(always)]
    pub fn qei_ctl_resmode(&self) -> QEI_CTL_RESMODE_R {
        QEI_CTL_RESMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture Velocity"]
    #[inline(always)]
    pub fn qei_ctl_velen(&self) -> QEI_CTL_VELEN_R {
        QEI_CTL_VELEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Predivide Velocity"]
    #[inline(always)]
    pub fn qei_ctl_veldiv(&self) -> QEI_CTL_VELDIV_R {
        QEI_CTL_VELDIV_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - Invert PhA"]
    #[inline(always)]
    pub fn qei_ctl_inva(&self) -> QEI_CTL_INVA_R {
        QEI_CTL_INVA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Invert PhB"]
    #[inline(always)]
    pub fn qei_ctl_invb(&self) -> QEI_CTL_INVB_R {
        QEI_CTL_INVB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Invert Index Pulse"]
    #[inline(always)]
    pub fn qei_ctl_invi(&self) -> QEI_CTL_INVI_R {
        QEI_CTL_INVI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Stall QEI"]
    #[inline(always)]
    pub fn qei_ctl_stallen(&self) -> QEI_CTL_STALLEN_R {
        QEI_CTL_STALLEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Input Filter"]
    #[inline(always)]
    pub fn qei_ctl_filten(&self) -> QEI_CTL_FILTEN_R {
        QEI_CTL_FILTEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Input Filter Prescale Count"]
    #[inline(always)]
    pub fn qei_ctl_filtcnt(&self) -> QEI_CTL_FILTCNT_R {
        QEI_CTL_FILTCNT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable QEI"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ctl_enable(&mut self) -> QEI_CTL_ENABLE_W<CTL_SPEC, 0> {
        QEI_CTL_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Swap Signals"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ctl_swap(&mut self) -> QEI_CTL_SWAP_W<CTL_SPEC, 1> {
        QEI_CTL_SWAP_W::new(self)
    }
    #[doc = "Bit 2 - Signal Mode"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ctl_sigmode(&mut self) -> QEI_CTL_SIGMODE_W<CTL_SPEC, 2> {
        QEI_CTL_SIGMODE_W::new(self)
    }
    #[doc = "Bit 3 - Capture Mode"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ctl_capmode(&mut self) -> QEI_CTL_CAPMODE_W<CTL_SPEC, 3> {
        QEI_CTL_CAPMODE_W::new(self)
    }
    #[doc = "Bit 4 - Reset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ctl_resmode(&mut self) -> QEI_CTL_RESMODE_W<CTL_SPEC, 4> {
        QEI_CTL_RESMODE_W::new(self)
    }
    #[doc = "Bit 5 - Capture Velocity"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ctl_velen(&mut self) -> QEI_CTL_VELEN_W<CTL_SPEC, 5> {
        QEI_CTL_VELEN_W::new(self)
    }
    #[doc = "Bits 6:8 - Predivide Velocity"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ctl_veldiv(&mut self) -> QEI_CTL_VELDIV_W<CTL_SPEC, 6> {
        QEI_CTL_VELDIV_W::new(self)
    }
    #[doc = "Bit 9 - Invert PhA"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ctl_inva(&mut self) -> QEI_CTL_INVA_W<CTL_SPEC, 9> {
        QEI_CTL_INVA_W::new(self)
    }
    #[doc = "Bit 10 - Invert PhB"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ctl_invb(&mut self) -> QEI_CTL_INVB_W<CTL_SPEC, 10> {
        QEI_CTL_INVB_W::new(self)
    }
    #[doc = "Bit 11 - Invert Index Pulse"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ctl_invi(&mut self) -> QEI_CTL_INVI_W<CTL_SPEC, 11> {
        QEI_CTL_INVI_W::new(self)
    }
    #[doc = "Bit 12 - Stall QEI"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ctl_stallen(&mut self) -> QEI_CTL_STALLEN_W<CTL_SPEC, 12> {
        QEI_CTL_STALLEN_W::new(self)
    }
    #[doc = "Bit 13 - Enable Input Filter"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ctl_filten(&mut self) -> QEI_CTL_FILTEN_W<CTL_SPEC, 13> {
        QEI_CTL_FILTEN_W::new(self)
    }
    #[doc = "Bits 16:19 - Input Filter Prescale Count"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ctl_filtcnt(&mut self) -> QEI_CTL_FILTCNT_W<CTL_SPEC, 16> {
        QEI_CTL_FILTCNT_W::new(self)
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
#[doc = "QEI Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
