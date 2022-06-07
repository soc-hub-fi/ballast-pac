#[doc = "Register `l2_enable` reader"]
pub struct R(crate::R<L2_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `l2_enable` writer"]
pub struct W(crate::W<L2_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L2_ENABLE_SPEC>;
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
impl From<crate::W<L2_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L2_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `l2_enable` reader - "]
pub struct L2_ENABLE_R(crate::FieldReader<u64>);
impl L2_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        L2_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L2_ENABLE_R {
    type Target = crate::FieldReader<u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `l2_enable` writer - "]
pub struct L2_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> L2_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u64) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn l2_enable(&self) -> L2_ENABLE_R {
        L2_ENABLE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn l2_enable(&mut self) -> L2_ENABLE_W {
        L2_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable L2 cache\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_enable](index.html) module"]
pub struct L2_ENABLE_SPEC;
impl crate::RegisterSpec for L2_ENABLE_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [l2_enable::R](R) reader structure"]
impl crate::Readable for L2_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l2_enable::W](W) writer structure"]
impl crate::Writable for L2_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets l2_enable to value 0"]
impl crate::Resettable for L2_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
