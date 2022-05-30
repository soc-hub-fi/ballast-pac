#[doc = "Register `REG_FILT` reader"]
pub struct R(crate::R<REG_FILT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_FILT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_FILT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_FILT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_FILT` writer"]
pub struct W(crate::W<REG_FILT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_FILT_SPEC>;
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
impl From<crate::W<REG_FILT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_FILT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_FILT` reader - "]
pub struct REG_FILT_R(crate::FieldReader<u32>);
impl REG_FILT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        REG_FILT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_FILT_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_FILT` writer - "]
pub struct REG_FILT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_FILT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_filt(&self) -> REG_FILT_R {
        REG_FILT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_filt(&mut self) -> REG_FILT_W {
        REG_FILT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER control mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_filt](index.html) module"]
pub struct REG_FILT_SPEC;
impl crate::RegisterSpec for REG_FILT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_filt::R](R) reader structure"]
impl crate::Readable for REG_FILT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_filt::W](W) writer structure"]
impl crate::Writable for REG_FILT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_FILT to value 0"]
impl crate::Resettable for REG_FILT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
