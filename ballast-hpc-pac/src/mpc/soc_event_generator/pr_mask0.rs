#[doc = "Register `PR_MASK0` reader"]
pub struct R(crate::R<PR_MASK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_MASK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_MASK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_MASK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR_MASK0` writer"]
pub struct W(crate::W<PR_MASK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_MASK0_SPEC>;
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
impl From<crate::W<PR_MASK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_MASK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR_MASK0` reader - "]
pub struct PR_MASK0_R(crate::FieldReader<u32>);
impl PR_MASK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PR_MASK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR_MASK0_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR_MASK0` writer - "]
pub struct PR_MASK0_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_MASK0_W<'a> {
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
    pub fn pr_mask0(&self) -> PR_MASK0_R {
        PR_MASK0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pr_mask0(&mut self) -> PR_MASK0_W {
        PR_MASK0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Events 0-31 dispatch mask to peripherals\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr_mask0](index.html) module"]
pub struct PR_MASK0_SPEC;
impl crate::RegisterSpec for PR_MASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr_mask0::R](R) reader structure"]
impl crate::Readable for PR_MASK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr_mask0::W](W) writer structure"]
impl crate::Writable for PR_MASK0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PR_MASK0 to value 0xffff_ffff"]
impl crate::Resettable for PR_MASK0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
