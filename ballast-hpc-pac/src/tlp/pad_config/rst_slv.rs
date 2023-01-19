#[doc = "Register `rst_slv` reader"]
pub struct R(crate::R<RST_SLV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_SLV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_SLV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_SLV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rst_slv` writer"]
pub struct W(crate::W<RST_SLV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_SLV_SPEC>;
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
impl From<crate::W<RST_SLV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_SLV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `drive_strength` reader - "]
pub type DRIVE_STRENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `drive_strength` writer - "]
pub type DRIVE_STRENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, RST_SLV_SPEC, u8, u8, 2, O>;
#[doc = "Field `output_enable` reader - "]
pub type OUTPUT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `pull_enable` reader - "]
pub type PULL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `pull_enable` writer - "]
pub type PULL_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RST_SLV_SPEC, bool, O>;
#[doc = "Field `pull_down_up` reader - Pull down 0 Pull up 1"]
pub type PULL_DOWN_UP_R = crate::BitReader<bool>;
#[doc = "Field `pull_down_up` writer - Pull down 0 Pull up 1"]
pub type PULL_DOWN_UP_W<'a, const O: u8> = crate::BitWriter<'a, u16, RST_SLV_SPEC, bool, O>;
#[doc = "Field `input_enable` reader - "]
pub type INPUT_ENABLE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn drive_strength(&self) -> DRIVE_STRENGTH_R {
        DRIVE_STRENGTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn output_enable(&self) -> OUTPUT_ENABLE_R {
        OUTPUT_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pull_enable(&self) -> PULL_ENABLE_R {
        PULL_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pull down 0 Pull up 1"]
    #[inline(always)]
    pub fn pull_down_up(&self) -> PULL_DOWN_UP_R {
        PULL_DOWN_UP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn input_enable(&self) -> INPUT_ENABLE_R {
        INPUT_ENABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn drive_strength(&mut self) -> DRIVE_STRENGTH_W<0> {
        DRIVE_STRENGTH_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pull_enable(&mut self) -> PULL_ENABLE_W<7> {
        PULL_ENABLE_W::new(self)
    }
    #[doc = "Bit 8 - Pull down 0 Pull up 1"]
    #[inline(always)]
    #[must_use]
    pub fn pull_down_up(&mut self) -> PULL_DOWN_UP_W<8> {
        PULL_DOWN_UP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_slv](index.html) module"]
pub struct RST_SLV_SPEC;
impl crate::RegisterSpec for RST_SLV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rst_slv::R](R) reader structure"]
impl crate::Readable for RST_SLV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst_slv::W](W) writer structure"]
impl crate::Writable for RST_SLV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rst_slv to value 0x8008"]
impl crate::Resettable for RST_SLV_SPEC {
    const RESET_VALUE: Self::Ux = 0x8008;
}
