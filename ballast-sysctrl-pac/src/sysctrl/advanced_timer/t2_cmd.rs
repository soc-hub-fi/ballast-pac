#[doc = "Register `T2_CMD` reader"]
pub struct R(crate::R<T2_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T2_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T2_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T2_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T2_CMD` writer"]
pub struct W(crate::W<T2_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T2_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<T2_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T2_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - ADV_TIMER2 start command bitfield"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - ADV_TIMER2 start command bitfield"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, T2_CMD_SPEC, bool, O>;
#[doc = "Field `STOP` reader - ADV_TIMER2 stop command bitfield."]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - ADV_TIMER2 stop command bitfield."]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, T2_CMD_SPEC, bool, O>;
#[doc = "Field `UPDATE` reader - ADV_TIMER2 update command bitfield."]
pub type UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `UPDATE` writer - ADV_TIMER2 update command bitfield."]
pub type UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, T2_CMD_SPEC, bool, O>;
#[doc = "Field `RESET` reader - ADV_TIMER2 reset command bitfield"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - ADV_TIMER2 reset command bitfield"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, T2_CMD_SPEC, bool, O>;
#[doc = "Field `ARM` reader - ADV_TIMER2 arm command bitfield."]
pub type ARM_R = crate::BitReader<bool>;
#[doc = "Field `ARM` writer - ADV_TIMER2 arm command bitfield."]
pub type ARM_W<'a, const O: u8> = crate::BitWriter<'a, u32, T2_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADV_TIMER2 start command bitfield"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADV_TIMER2 stop command bitfield."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADV_TIMER2 update command bitfield."]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADV_TIMER2 reset command bitfield"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADV_TIMER2 arm command bitfield."]
    #[inline(always)]
    pub fn arm(&self) -> ARM_R {
        ARM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADV_TIMER2 start command bitfield"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - ADV_TIMER2 stop command bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<1> {
        STOP_W::new(self)
    }
    #[doc = "Bit 2 - ADV_TIMER2 update command bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<2> {
        UPDATE_W::new(self)
    }
    #[doc = "Bit 3 - ADV_TIMER2 reset command bitfield"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<3> {
        RESET_W::new(self)
    }
    #[doc = "Bit 4 - ADV_TIMER2 arm command bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn arm(&mut self) -> ARM_W<4> {
        ARM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADV_TIMER2 command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t2_cmd](index.html) module"]
pub struct T2_CMD_SPEC;
impl crate::RegisterSpec for T2_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t2_cmd::R](R) reader structure"]
impl crate::Readable for T2_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t2_cmd::W](W) writer structure"]
impl crate::Writable for T2_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T2_CMD to value 0"]
impl crate::Resettable for T2_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
