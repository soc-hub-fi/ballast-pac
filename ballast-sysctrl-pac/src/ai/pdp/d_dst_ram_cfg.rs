#[doc = "Register `D_DST_RAM_CFG` reader"]
pub struct R(crate::R<D_DST_RAM_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DST_RAM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DST_RAM_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DST_RAM_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DST_RAM_CFG` writer"]
pub struct W(crate::W<D_DST_RAM_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DST_RAM_CFG_SPEC>;
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
impl From<crate::W<D_DST_RAM_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DST_RAM_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_RAM_TYPE_A {
    #[doc = "0: `0`"]
    CV = 0,
    #[doc = "1: `1`"]
    MC = 1,
}
impl From<DST_RAM_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: DST_RAM_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DST_RAM_TYPE` reader - "]
pub struct DST_RAM_TYPE_R(crate::FieldReader<bool, DST_RAM_TYPE_A>);
impl DST_RAM_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DST_RAM_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_RAM_TYPE_A {
        match self.bits {
            false => DST_RAM_TYPE_A::CV,
            true => DST_RAM_TYPE_A::MC,
        }
    }
    #[doc = "Checks if the value of the field is `CV`"]
    #[inline(always)]
    pub fn is_cv(&self) -> bool {
        **self == DST_RAM_TYPE_A::CV
    }
    #[doc = "Checks if the value of the field is `MC`"]
    #[inline(always)]
    pub fn is_mc(&self) -> bool {
        **self == DST_RAM_TYPE_A::MC
    }
}
impl core::ops::Deref for DST_RAM_TYPE_R {
    type Target = crate::FieldReader<bool, DST_RAM_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DST_RAM_TYPE` writer - "]
pub struct DST_RAM_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_RAM_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_RAM_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn cv(self) -> &'a mut W {
        self.variant(DST_RAM_TYPE_A::CV)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mc(self) -> &'a mut W {
        self.variant(DST_RAM_TYPE_A::MC)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dst_ram_type(&self) -> DST_RAM_TYPE_R {
        DST_RAM_TYPE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dst_ram_type(&mut self) -> DST_RAM_TYPE_W {
        DST_RAM_TYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM type of destination cube\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dst_ram_cfg](index.html) module"]
pub struct D_DST_RAM_CFG_SPEC;
impl crate::RegisterSpec for D_DST_RAM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dst_ram_cfg::R](R) reader structure"]
impl crate::Readable for D_DST_RAM_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_dst_ram_cfg::W](W) writer structure"]
impl crate::Writable for D_DST_RAM_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_DST_RAM_CFG to value 0"]
impl crate::Resettable for D_DST_RAM_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
