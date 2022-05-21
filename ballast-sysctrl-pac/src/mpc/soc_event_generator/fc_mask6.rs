#[doc = "Register `FC_MASK6` reader"]
pub struct R(crate::R<FC_MASK6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FC_MASK6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FC_MASK6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FC_MASK6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FC_MASK6` writer"]
pub struct W(crate::W<FC_MASK6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FC_MASK6_SPEC>;
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
impl From<crate::W<FC_MASK6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FC_MASK6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FC_MASK6` reader - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
pub struct FC_MASK6_R(crate::FieldReader<u32, u32>);
impl FC_MASK6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FC_MASK6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC_MASK6_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC_MASK6` writer - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
pub struct FC_MASK6_W<'a> {
    w: &'a mut W,
}
impl<'a> FC_MASK6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
    #[inline(always)]
    pub fn fc_mask6(&self) -> FC_MASK6_R {
        FC_MASK6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Event Mask to enable/disable event dispatch to FC interrupt controller. Setting bit\\[i\\]
to 0b1 disables dispatching event to FC interrupt controller. Setting bit\\[i\\]
to 0b0 enables dispatching event to FC interrupt controller."]
    #[inline(always)]
    pub fn fc_mask6(&mut self) -> FC_MASK6_W {
        FC_MASK6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Events 191-223 dispatch mask to FC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fc_mask6](index.html) module"]
pub struct FC_MASK6_SPEC;
impl crate::RegisterSpec for FC_MASK6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fc_mask6::R](R) reader structure"]
impl crate::Readable for FC_MASK6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fc_mask6::W](W) writer structure"]
impl crate::Writable for FC_MASK6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FC_MASK6 to value 0xffff_ffff"]
impl crate::Resettable for FC_MASK6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
